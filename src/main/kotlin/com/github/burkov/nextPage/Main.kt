package com.github.burkov.nextPage

import com.github.kittinunf.fuel.httpGet
import com.github.kittinunf.fuel.httpPost
import org.apache.commons.text.StringEscapeUtils
import org.jsoup.Jsoup
import java.nio.file.Files
import java.nio.file.Paths
import java.awt.Toolkit.getDefaultToolkit
import java.awt.datatransfer.StringSelection



data class Tokens(val csrf: String, val cookie: String) {
    companion object {
        fun fetch(): Tokens {
            val (_, resp, result) = "https://recommendmeabook.com/".httpGet().responseString()
            require(resp.statusCode == 200)
            return Tokens(
                csrf = Jsoup.parse(result.get()).select("meta[name=csrf-token]").attr("content"),
                cookie = resp.header("Set-Cookie").single().substringBefore(";")
            )
        }
    }

    fun toHeaders(): Map<String, String> = mapOf(
        "X-CSRF-Token" to csrf,
        "Cookie" to cookie
    )
}

object CurrentBooknumber {
    private val dataDirPath = Paths.get(System.getProperty("user.home"), ".local", "share", "nextPage")
    private val file = dataDirPath.resolve("booknumber").toFile()

    init {
        Files.createDirectories(dataDirPath)
        file.takeIf { !it.exists() }?.let {
            require(it.createNewFile())
            it.writeText("0")
        }
    }

    fun get(): Int {
        return file.readText().toInt()
    }

    fun set(n: Int) {
        file.writeText(n.toString())
    }
}

fun main() {
    val tokens = Tokens.fetch()
    var n = CurrentBooknumber.get()
    try {
        loop@ while (true) {
            val (_, response, result) = "https://www.recommendmeabook.com/home/next_book"
                .httpPost(listOf("booknumber" to n++))
                .header(tokens.toHeaders())
                .header("Accept" to "*/*")
                .responseString()
            when (response.statusCode) {
                404 -> {
                    println("Missing book #${n - 1}")
                    continue@loop
                }
                200 -> {
                    val map = result.get().split(";\n").filter { it.contains("var title") || it.contains("var page") }
                        .map {
                            val key = it.substringBefore("=").trim().removePrefix("var").trim()
                            val value = Jsoup.parse(StringEscapeUtils.unescapeJava(it.substringAfter("=").trim()))
                                .text()
                                .replace(Regex("[^\\p{Print}]"), "?")
                                .removeSurrounding("\"")
                                .trim()
                            key to value
                        }
                        .toMap()
                    val text = map.getValue("page")
                    val selection = StringSelection(text)
                    getDefaultToolkit().systemClipboard.setContents(selection, selection)
                    println("Booknum: $n")
                    println("Title  : ${map["title"]}")
                    println("Symbols: ${text.length}")
                    println("Words  : ${text.count { it.isWhitespace() }}")
                    (60 downTo 0).forEach { i ->
                        print("\rYou have $i seconds to paste text      ")
                        Thread.sleep(1000)
                    }
                    println()
                    break@loop
                }
                else -> error("unexpected code = ${response.statusCode}")
            }
        }
    } finally {
        CurrentBooknumber.set(n)
    }
}