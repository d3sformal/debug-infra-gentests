#!/usr/bin/env bash

set -x

javac targets/extraction/*.java
jar cmvf targets/extraction/MANIFEST.MF targets/extraction/Test.jar targets/extraction/*.class
