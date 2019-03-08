#!/bin/bash

./gradlew shadowJar
java -jar build/libs/nextPage-1.0-SNAPSHOT-all.jar
