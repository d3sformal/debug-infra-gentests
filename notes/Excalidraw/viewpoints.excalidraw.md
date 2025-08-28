---

excalidraw-plugin: parsed
tags: [excalidraw]

---
==⚠  Switch to EXCALIDRAW VIEW in the MORE OPTIONS menu of this document. ⚠==


# Text Elements
Module Viewpoint - Gradle Project Structure ^RhsrOLSt

runner ^jTzmIY5X

model-java ^1jBFuHJn

instrumentor-common ^1qA27gcr

instrumentor-java ^yhQy6xWL

analyzer-common ^8QYx4syD

test-generator-common ^vnPMBuyk

test-generator-java ^pDmPXMl6

Main ^PKfSwCmg

Orchestrates the workflow. ^SVgGgN8I

Acts as the main entry point (CLI/GUI). ^3FgEcpkR

Identifiers ^YgXabvqd

Trace ^0Vaf6Zq1

Defines the central ^N3enlkGe

data model for Java entities. ^iUfddnnE

Instrumentor ^laIX1LOq

Abstract instrumentor ^4o4HZ8RF

interfaces and logic. ^ffgcI0fO

DiSLInstrumentor ^5KFwDn3k

Java-specific implementation ^cInqC4tC

using the DiSL framework. ^A5Ant9xG

Analyzer ^8zxikTJn

Abstract analyzer interfaces and logic. ^yWWKnSVG

UnitTestGenerator ^eb9xwTAA

Abstract test generator ^4N0Tgkkt

interfaces and logic. ^eiMzhTrQ

TraceBasedUnitTestGenerator ^bPYBTPLv

Java-specific implementation ^QguHerbm

that generates tests from traces. ^clp0Ynfb

implements ^A8qkALfH

implements ^QzX5exJp

DiSL ^Xreku4w4

Instrumentation
(DiSLClass) ^gOuLsmYi

Static Context ^VAvpe8TI

Guards ^7gUEcdOf

Markers ^bLl4d3Im

Target App
(JAR) ^xNhDIm9D

GENERATED ^n8YMpQbv

Arguments
for launching
DiSL ^hEJBZ9do

Exit Code ^CquY9FUs

Instrumentation 
code execution

(prints to stdout, 
stderr, writing to 
sockets, etc.) ^lYvrkIhM

DiSL Workflow ^f1n31kf4

Target App 
(JAR) ^CvRWN7el

Target Method Identifier ^Pj2R8O1q

Level of Instrumentation ^CNetsXkr

Input ^aWeG3tZi

runner ^es7ralVr

ArgumentParser ^igbUBQ7y

instrumentor-common ^XVL5dYm1

instrumentor-java ^k5Cp1Z2K

instrumentor-* ^qFbGf4DL

... ^MF52ARC8

... ^v2Z2YevL

... ^9E1ebEIT

analyzer-disl ^Y0UCHtcc

DiSLClass ^x1gzOHdi

Static Context ^MvSzbIdZ

Guards ^SWptRktW

Markers ^4lDE5qH3

ResultCollector ^HBDpc5do

test-generator-common ^aZ7QBtj1

test-generator-java ^7oQWMrp0

test-generator-* ^BOHUxaAg

... ^Xeq70799

... ^C33mBKb8

... ^7KzAcbwK

test-runner-common ^vxR8iO4V

test-runner-java ^DrBMTT0b

... ^h0Uqs2oD

... ^sDigU35S

... ^46xMP4n1

test-runner-* ^M97Zfzpi

UI ^oxjul6ug

TraceVisualizer ^WZGOceJK

Test Visualizer ^NwpYt9oG

InstGenerator ^xNPxWR71

InstCompiler ^4b7T4Yfi

implements ^CXLKzc7A

User ^e0BfIBhn

Auto-Debugger Core Components ^Y0zuSBnx

Java Instrumentor ^qjEfp259

(from `instrumentor-java`) ^Aq1KmIiK

Java Analyzer ^VIe6d3H2

(from `analyzer-java`) ^RJJ6Y8Fe

Java Test Generator ^rMOQXxM3

(from `test-generator-java`) ^MNXjsA0l

analyzer-java ^yssLmoop

Collector ^SDzMdPm5

Concrete analyzer that processes ^2Iseuy11

instrumentation output from DiSL. ^dnB5CUet

Instrumentation & Communication Frameworks ^QKCLfsXG

DiSL Framework ^oyN37qQE

ShadowVM ^mfdOMRdX

(Communication Channel) ^heoUEDFU

Runner / Orchestrator ^ExXRdwXk

(from `runner` module) ^VbiL0CIV

Target Application ^GvMDbHyU

(User's APK) ^ypfzE3fN

1. Submits Run Config ^ykwuFh5b

2. Invokes
with Config ^MZbH9J7Y

3. Generates Instrumentation ^9iPlzMCs

4. Runs Instrumentation ^V7skKOZT

5. Instruments Bytecode at Runtime ^fqACL8OG

6. Emits Runtime Values ^SwUE28Vi

7. Consumes Runtime Values ^JJeLkZAa

8. Sends collected Execution Trace 
 ^XaUH5Yed

10. Displays generated tests & status ^eFVbfjjm

Executes
tests ^13wJI0qJ

Runs tests against application ^NfaekTNL

Components & Connectors Viewpoint  ^S8i1vDXv

9. Returns Generated Test Suite ^qiQS1G8S

%%
# Drawing
```json
{
	"type": "excalidraw",
	"version": 2,
	"source": "https://github.com/zsviczian/obsidian-excalidraw-plugin/releases/tag/2.1.3",
	"elements": [
		{
			"type": "rectangle",
			"version": 308,
			"versionNonce": 1468669961,
			"isDeleted": false,
			"id": "awpSRCM4P6OlCB7DnHGCZ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -1002.6992568869066,
			"y": -585.5993421027691,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1478.4260063070726,
			"height": 795.8494073755561,
			"seed": 967033703,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747763,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 97,
			"versionNonce": 1437499655,
			"isDeleted": false,
			"id": "RhsrOLSt",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -515.7738973473511,
			"y": -637.8652995449274,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 428.806640625,
			"height": 33,
			"seed": 1035592775,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747763,
			"link": null,
			"locked": false,
			"fontSize": 22,
			"fontFamily": 2,
			"text": "Module Viewpoint - Gradle Project Structure",
			"rawText": "Module Viewpoint - Gradle Project Structure",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Module Viewpoint - Gradle Project Structure",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 506,
			"versionNonce": 54212329,
			"isDeleted": false,
			"id": "O2BRoRvJxzeTrGsV39kGg",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -514.9124381337932,
			"y": -563.7573384529252,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "#FFFFFF",
			"width": 448,
			"height": 113.30000305175781,
			"seed": 156849575,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "4UCu8L5gOj6Ea4PZe6ufx",
					"type": "arrow"
				},
				{
					"id": "_r84BolUeTvk_bHGSi1n9",
					"type": "arrow"
				},
				{
					"id": "TEkzmNrvhAUC1h4nbJbmc",
					"type": "arrow"
				}
			],
			"updated": 1750834747763,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 501,
			"versionNonce": 1248249895,
			"isDeleted": false,
			"id": "jTzmIY5X",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -317.51454995019947,
			"y": -562.7622365242142,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 40.46875,
			"height": 21,
			"seed": 923666631,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747763,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "runner",
			"rawText": "runner",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "runner",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 941,
			"versionNonce": 174485961,
			"isDeleted": false,
			"id": "Ay3kuijPCSvrHWLyQuoPB",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -536.9754383360455,
			"y": 81.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 533,
			"height": 113.30000305175781,
			"seed": 771708903,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "24sSnWe7NVy1pYvCvt5oh",
					"type": "arrow"
				},
				{
					"id": "_QVnMz1lfNs0yo7gMSd9N",
					"type": "arrow"
				}
			],
			"updated": 1750834747763,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 899,
			"versionNonce": 180955975,
			"isDeleted": false,
			"id": "1jBFuHJn",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -314.0511524473736,
			"y": 82.74847257855805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 68.4755859375,
			"height": 21,
			"seed": 1413071623,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "_QVnMz1lfNs0yo7gMSd9N",
					"type": "arrow"
				},
				{
					"id": "gBr8RPfhg9T8G2uo_6b6Y",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "model-java",
			"rawText": "model-java",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "model-java",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 516,
			"versionNonce": 1109872809,
			"isDeleted": false,
			"id": "AACdf80OpYKzPzCPtOhfl",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -961.8850244388101,
			"y": -370.2471103341428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 399,
			"height": 113.29000091552734,
			"seed": 439994919,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "JuZbzD2cW6JgoA4prZ1Te",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 513,
			"versionNonce": 1395796583,
			"isDeleted": false,
			"id": "1qA27gcr",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -850.592513452482,
			"y": -369.2520236642209,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 136.1513671875,
			"height": 21,
			"seed": 753659207,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "4UCu8L5gOj6Ea4PZe6ufx",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "instrumentor-common",
			"rawText": "instrumentor-common",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "instrumentor-common",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 485,
			"versionNonce": 1344885641,
			"isDeleted": false,
			"id": "haTpj4rOqeLkWvnsUh7PH",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -987.7394706860207,
			"y": -106.14008200242358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 474,
			"height": 113.30000305175781,
			"seed": 1739369575,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "24sSnWe7NVy1pYvCvt5oh",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 481,
			"versionNonce": 544097671,
			"isDeleted": false,
			"id": "yhQy6xWL",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -822.0177604809426,
			"y": -105.14496481492358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 108.1513671875,
			"height": 21,
			"seed": 105416583,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "JuZbzD2cW6JgoA4prZ1Te",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "instrumentor-java",
			"rawText": "instrumentor-java",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "instrumentor-java",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 486,
			"versionNonce": 413661801,
			"isDeleted": false,
			"id": "6iq3uEk34t_ASRCYzQEzX",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -513.7855349906368,
			"y": -372.09019152582334,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 448,
			"height": 113.29000091552734,
			"seed": 385613479,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "jmbEZGJVbPLZ2KtJrb5_I",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 494,
			"versionNonce": 565836967,
			"isDeleted": false,
			"id": "8QYx4syD",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -340.3338137992306,
			"y": -371.09510485590147,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 111.26171875,
			"height": 21,
			"seed": 745771463,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "TEkzmNrvhAUC1h4nbJbmc",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "analyzer-common",
			"rawText": "analyzer-common",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "analyzer-common",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 472,
			"versionNonce": 1819198793,
			"isDeleted": false,
			"id": "ed0IN-cIZou1l4l0PGaVp",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -10.63424661074282,
			"y": -370.6066494697428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 445,
			"height": 113.29000091552734,
			"seed": 1338753255,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "zGXnobSRfU3AUZvNbeVgV",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 469,
			"versionNonce": 1791086535,
			"isDeleted": false,
			"id": "vnPMBuyk",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 117.83743307675718,
			"y": -369.61156279982094,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 145.5029296875,
			"height": 21,
			"seed": 848366599,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "_r84BolUeTvk_bHGSi1n9",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "test-generator-common",
			"rawText": "test-generator-common",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "test-generator-common",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 567,
			"versionNonce": 1993946153,
			"isDeleted": false,
			"id": "_TI2H9_KkXXRx1-U-5Rb5",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -35.48104608026341,
			"y": -105.21471776713497,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 496.38645432462175,
			"height": 113.30000305175781,
			"seed": 549450535,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "_QVnMz1lfNs0yo7gMSd9N",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 524,
			"versionNonce": 1705131751,
			"isDeleted": false,
			"id": "pDmPXMl6",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 88.30628715060834,
			"y": -104.21960057963497,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 117.5029296875,
			"height": 21,
			"seed": 851768903,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "zGXnobSRfU3AUZvNbeVgV",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "test-generator-java",
			"rawText": "test-generator-java",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "test-generator-java",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 503,
			"versionNonce": 1808975625,
			"isDeleted": false,
			"id": "md1ENvCKWjBn-J1Qk82Wx",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -490.62242958887134,
			"y": -520.7573308235307,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "#FFFFFF",
			"width": 73.4208984375,
			"height": 46.29690170288086,
			"seed": 1065129319,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 502,
			"versionNonce": 82299399,
			"isDeleted": false,
			"id": "Lxm7RT4uadARGlwEqcrFF",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -437.20153115137134,
			"y": -515.7573308235307,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "#FFFFFF",
			"width": 15,
			"height": 10,
			"seed": 238243975,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 502,
			"versionNonce": 1977023977,
			"isDeleted": false,
			"id": "ZlUfutp2nPjePaA_c45mn",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -439.20153115137134,
			"y": -513.7573308235307,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "#FFFFFF",
			"width": 4,
			"height": 2,
			"seed": 1218815911,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 502,
			"versionNonce": 898560295,
			"isDeleted": false,
			"id": "9XNayNfduoGyie5RHZ9aJ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -439.20153115137134,
			"y": -509.75733082353065,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "#FFFFFF",
			"width": 4,
			"height": 2,
			"seed": 754981575,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 501,
			"versionNonce": 1676761289,
			"isDeleted": false,
			"id": "PKfSwCmg",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -475.62242958887134,
			"y": -501.7622288948197,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 30.3447265625,
			"height": 21,
			"seed": 2145331687,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Main",
			"rawText": "Main",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Main",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 502,
			"versionNonce": 285488199,
			"isDeleted": false,
			"id": "dCgCPpJNrVV8-AFwRIMqZ",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -381.9224478994182,
			"y": -517.7373418098588,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "#FFFFFF",
			"width": 307.0380859375,
			"height": 40.26560974121094,
			"seed": 1045386503,
			"groupIds": [
				"hDNYlyqOAP7l19JKcvBIF"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					16.1300048828125
				],
				[
					-35.019989013671875,
					20.1300048828125
				],
				[
					0,
					24.1300048828125
				],
				[
					0,
					40.26560974121094
				],
				[
					0,
					40.26560974121094
				],
				[
					0,
					40.26560974121094
				],
				[
					272.0180969238281,
					40.26560974121094
				],
				[
					272.0180969238281,
					40.26560974121094
				],
				[
					272.0180969238281,
					40.26560974121094
				],
				[
					272.0180969238281,
					10
				],
				[
					262.0180969238281,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 502,
			"versionNonce": 398284713,
			"isDeleted": false,
			"id": "YEjdgYqf0x_iHlQKpbtpY",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -119.90435097559009,
			"y": -517.7373418098588,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "#FFFFFF",
			"width": 10,
			"height": 10,
			"seed": 265100327,
			"groupIds": [
				"re-VMg4iuCLTN3evA9PUb"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					10
				],
				[
					10,
					10
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 501,
			"versionNonce": 1327764327,
			"isDeleted": false,
			"id": "SVgGgN8I",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -375.9224478994182,
			"y": -513.6704320198197,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 153.17529296875,
			"height": 19.5,
			"seed": 2017787719,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Orchestrates the workflow.",
			"rawText": "Orchestrates the workflow.",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Orchestrates the workflow.",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 502,
			"versionNonce": 862354057,
			"isDeleted": false,
			"id": "3FgEcpkR",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -375.9224478994182,
			"y": -498.5376347786088,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 223.2470703125,
			"height": 19.5,
			"seed": 1242791527,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "_r84BolUeTvk_bHGSi1n9",
					"type": "arrow"
				}
			],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Acts as the main entry point (CLI/GUI).",
			"rawText": "Acts as the main entry point (CLI/GUI).",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Acts as the main entry point (CLI/GUI).",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 898,
			"versionNonce": 2017136263,
			"isDeleted": false,
			"id": "JICPkiIJLJSsSOjj1zI4x",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -158.15546152940487,
			"y": 124.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 110.36229705810547,
			"height": 46.29690170288086,
			"seed": 1476729223,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 897,
			"versionNonce": 128467305,
			"isDeleted": false,
			"id": "uNtmCT3ELf6FQxkdgfsdZ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -67.79315684190487,
			"y": 129.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 1733709991,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 897,
			"versionNonce": 1459987879,
			"isDeleted": false,
			"id": "s6vgEDZrkQ_j2uV49EPrm",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -69.79315684190487,
			"y": 131.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1639162823,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 897,
			"versionNonce": 1104030793,
			"isDeleted": false,
			"id": "J8-iRlCUd1WYjfAIu9TJi",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -69.79315684190487,
			"y": 135.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1194955495,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 895,
			"versionNonce": 716200135,
			"isDeleted": false,
			"id": "YgXabvqd",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -143.15546152940487,
			"y": 143.74847257855805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 60.6962890625,
			"height": 21,
			"seed": 1618255367,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Identifiers",
			"rawText": "Identifiers",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Identifiers",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 898,
			"versionNonce": 1504951081,
			"isDeleted": false,
			"id": "g5ty1oLBNvMLaOPpKr0q3",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -512.5754444395611,
			"y": 124.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 79.19730377197266,
			"height": 46.29690170288086,
			"seed": 811681063,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 897,
			"versionNonce": 416786407,
			"isDeleted": false,
			"id": "NpGeiBu_Yx_cgOT8kd4G5",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -453.378148296983,
			"y": 129.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 644614215,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 897,
			"versionNonce": 1069837833,
			"isDeleted": false,
			"id": "PgCkeSSQtCO-8KqDKfMq3",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -455.378148296983,
			"y": 131.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 275867495,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 897,
			"versionNonce": 1602753287,
			"isDeleted": false,
			"id": "uVpDWg_QetqGZniA-xaRN",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -455.378148296983,
			"y": 135.75335539105805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1287512711,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 895,
			"versionNonce": 1258013929,
			"isDeleted": false,
			"id": "0Vaf6Zq1",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -497.5754444395611,
			"y": 143.74847257855805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 35.2734375,
			"height": 21,
			"seed": 1007548839,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Trace",
			"rawText": "Trace",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Trace",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 897,
			"versionNonce": 1195252263,
			"isDeleted": false,
			"id": "Caw45WjFO2GFt-EkM-5zp",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -398.5654346739361,
			"y": 127.77337492230805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 239.83358764648438,
			"height": 40.26556396484375,
			"seed": 1938697415,
			"groupIds": [
				"PA0Q8OBQAkV9NBWUSE8Mv"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					16.1300048828125
				],
				[
					-34.649993896484375,
					20.1300048828125
				],
				[
					0,
					24.1300048828125
				],
				[
					0,
					40.26556396484375
				],
				[
					0,
					40.26556396484375
				],
				[
					0,
					40.26556396484375
				],
				[
					205.18359375,
					40.26556396484375
				],
				[
					205.18359375,
					40.26556396484375
				],
				[
					205.18359375,
					40.26556396484375
				],
				[
					205.18359375,
					10
				],
				[
					195.18359375,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 897,
			"versionNonce": 1726300105,
			"isDeleted": false,
			"id": "4IE7Mb3-WEqJEtruGz5c_",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -203.38184092393612,
			"y": 127.77337492230805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 10,
			"height": 10,
			"seed": 786103271,
			"groupIds": [
				"d2YaXKLRRd6PJVmqlgppu"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					10
				],
				[
					10,
					10
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 895,
			"versionNonce": 1424375111,
			"isDeleted": false,
			"id": "N3enlkGe",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -392.5654346739361,
			"y": 131.84026945355805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 108.392578125,
			"height": 19.5,
			"seed": 33613575,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Defines the central",
			"rawText": "Defines the central",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Defines the central",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 895,
			"versionNonce": 27025065,
			"isDeleted": false,
			"id": "iUfddnnE",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -392.5654346739361,
			"y": 146.97308195355805,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 162.5888671875,
			"height": 19.5,
			"seed": 1266445863,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "data model for Java entities.",
			"rawText": "data model for Java entities.",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "data model for Java entities.",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 515,
			"versionNonce": 1004338279,
			"isDeleted": false,
			"id": "r-tUsc70EXPof6V4Pn2Hl",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -937.5550226077554,
			"y": -327.2471103341428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 131.34860229492188,
			"height": 46.29690170288086,
			"seed": 1715081543,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 514,
			"versionNonce": 969712009,
			"isDeleted": false,
			"id": "RAL4LdcqyXVvAQIC1u5IM",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -826.2064355716226,
			"y": -322.2471103341428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 462835815,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 514,
			"versionNonce": 267310983,
			"isDeleted": false,
			"id": "j4c7fHM6qUYMCN35gZd-c",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -828.2064355716226,
			"y": -320.2471103341428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1432202119,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 514,
			"versionNonce": 1286703209,
			"isDeleted": false,
			"id": "IJgWXT1FSLrvn-vwxKEty",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -828.2064355716226,
			"y": -316.2471103341428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 265069223,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 512,
			"versionNonce": 928534183,
			"isDeleted": false,
			"id": "laIX1LOq",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -922.5550226077554,
			"y": -308.2519931466428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 78.5859375,
			"height": 21,
			"seed": 193967559,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Instrumentor",
			"rawText": "Instrumentor",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Instrumentor",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 514,
			"versionNonce": 1142330185,
			"isDeleted": false,
			"id": "DFys0McJPRUYgcNS0bZAn",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -771.0350183352945,
			"y": -324.2371310860959,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 199.03469848632812,
			"height": 40.265594482421875,
			"seed": 1069769959,
			"groupIds": [
				"VOwD59PeE9ZecWdTCal1u"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					16.130035400390625
				],
				[
					-34.730010986328125,
					20.130035400390625
				],
				[
					0,
					24.130035400390625
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					164.3046875,
					40.265594482421875
				],
				[
					164.3046875,
					40.265594482421875
				],
				[
					164.3046875,
					40.265594482421875
				],
				[
					164.3046875,
					10.000030517578125
				],
				[
					154.3046875,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 514,
			"versionNonce": 482730439,
			"isDeleted": false,
			"id": "5KXqlKp4WjknSb82HezgQ",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -616.7303308352945,
			"y": -324.2371310860959,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 10,
			"height": 10.000030517578125,
			"seed": 200552455,
			"groupIds": [
				"alY41KkSptQQw87dW6WgP"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					10.000030517578125
				],
				[
					10,
					10.000030517578125
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 512,
			"versionNonce": 1683440169,
			"isDeleted": false,
			"id": "4o4HZ8RF",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -765.0350183352945,
			"y": -320.1702060372678,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 123.54443359375,
			"height": 19.5,
			"seed": 706647847,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Abstract instrumentor",
			"rawText": "Abstract instrumentor",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Abstract instrumentor",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 512,
			"versionNonce": 559157479,
			"isDeleted": false,
			"id": "ffgcI0fO",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -765.0350183352945,
			"y": -305.0373935372678,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 115.62255859375,
			"height": 19.5,
			"seed": 1622088263,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "interfaces and logic.",
			"rawText": "interfaces and logic.",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "interfaces and logic.",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 483,
			"versionNonce": 1516180745,
			"isDeleted": false,
			"id": "O-DbvZU0mwXRhXTZW3il6",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -964.0894767895363,
			"y": -63.14008200242358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 162.70509338378906,
			"height": 46.29690170288086,
			"seed": 1954691431,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 482,
			"versionNonce": 1318018055,
			"isDeleted": false,
			"id": "L9twfsLvX9fAGwCuElu3k",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -821.3843681469582,
			"y": -58.14008200242358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 1524921479,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 482,
			"versionNonce": 698977257,
			"isDeleted": false,
			"id": "p4sqb0On5iJc7QdK3z8VB",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -823.3843681469582,
			"y": -56.14008200242358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 731007911,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 482,
			"versionNonce": 1215578919,
			"isDeleted": false,
			"id": "0boikWud3RS7mbDT3AeoU",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -823.3843681469582,
			"y": -52.14008200242358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1387806407,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 480,
			"versionNonce": 1514284745,
			"isDeleted": false,
			"id": "5KFwDn3k",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -949.0894767895363,
			"y": -44.14496481492358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 108.9306640625,
			"height": 21,
			"seed": 49466855,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "DiSLInstrumentor",
			"rawText": "DiSLInstrumentor",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "DiSLInstrumentor",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 482,
			"versionNonce": 2137998919,
			"isDeleted": false,
			"id": "uAZe1vAuMM6Xhm3UgxKbo",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -766.439482893052,
			"y": -60.12006247117358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 244.12481689453125,
			"height": 40.265594482421875,
			"seed": 1733196039,
			"groupIds": [
				"oaXdWObbr0zm1kAAebom3"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					16.129974365234375
				],
				[
					-34.720001220703125,
					20.129974365234375
				],
				[
					0,
					24.129974365234375
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					209.40481567382812,
					40.265594482421875
				],
				[
					209.40481567382812,
					40.265594482421875
				],
				[
					209.40481567382812,
					40.265594482421875
				],
				[
					209.40481567382812,
					10
				],
				[
					199.40481567382812,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 482,
			"versionNonce": 1724147113,
			"isDeleted": false,
			"id": "k3ncIJwJ02rP4Y66Ryjgn",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -567.0346672192238,
			"y": -60.12006247117358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 10,
			"height": 10,
			"seed": 357023783,
			"groupIds": [
				"vFsgFUaoV1ezYCCy8rOsm"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					10
				],
				[
					10,
					10
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 480,
			"versionNonce": 576513383,
			"isDeleted": false,
			"id": "cInqC4tC",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -760.439482893052,
			"y": -56.05316793992358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 166.9052734375,
			"height": 19.5,
			"seed": 1053500231,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Java-specific implementation",
			"rawText": "Java-specific implementation",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Java-specific implementation",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 480,
			"versionNonce": 1784400009,
			"isDeleted": false,
			"id": "A5Ant9xG",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -760.439482893052,
			"y": -40.92038595750171,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 152.45166015625,
			"height": 19.5,
			"seed": 607461991,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "using the DiSL framework.",
			"rawText": "using the DiSL framework.",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "using the DiSL framework.",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 484,
			"versionNonce": 1351196807,
			"isDeleted": false,
			"id": "fwcTrHamnOFczsezdCQ5y",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -489.90553010782435,
			"y": -329.09019152582334,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 102.24120330810547,
			"height": 46.29690170288086,
			"seed": 1179794823,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 483,
			"versionNonce": 247713641,
			"isDeleted": false,
			"id": "aRQ_yJTywINZ79glrciH-",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -407.66431917032435,
			"y": -324.09019152582334,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 1482618023,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 483,
			"versionNonce": 196212647,
			"isDeleted": false,
			"id": "1GCq5UxHBUz9AP4dnPHwU",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -409.66431917032435,
			"y": -322.09019152582334,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1913112519,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 483,
			"versionNonce": 977795657,
			"isDeleted": false,
			"id": "fjfMXanQ1EL4stFfjJth8",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -409.66431917032435,
			"y": -318.09019152582334,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 23672551,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 490,
			"versionNonce": 1778543303,
			"isDeleted": false,
			"id": "8zxikTJn",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -476.2712988592283,
			"y": -310.09507433832334,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 54.46875,
			"height": 21,
			"seed": 2098184711,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Analyzer",
			"rawText": "Analyzer",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Analyzer",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 483,
			"versionNonce": 2109983017,
			"isDeleted": false,
			"id": "stSJFPnGA6yeJxM_p_4k-",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -352.90553010782435,
			"y": -326.08021227777647,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 278.5244140625,
			"height": 40.265594482421875,
			"seed": 1442310439,
			"groupIds": [
				"S8sRaKl0lSt46KnODhyvi"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					16.130035400390625
				],
				[
					-34.28997802734375,
					20.130035400390625
				],
				[
					0,
					24.130035400390625
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					244.23443603515625,
					40.265594482421875
				],
				[
					244.23443603515625,
					40.265594482421875
				],
				[
					244.23443603515625,
					40.265594482421875
				],
				[
					244.23443603515625,
					10.000030517578125
				],
				[
					234.23443603515625,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 483,
			"versionNonce": 135756263,
			"isDeleted": false,
			"id": "p9WjExYCCKw7Sq0Fa6E_5",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -118.6710940726681,
			"y": -326.08021227777647,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 10,
			"height": 10.000030517578125,
			"seed": 697183303,
			"groupIds": [
				"KZskgUKU9c1vmZJbCu211"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747764,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					10.000030517578125
				],
				[
					10,
					10.000030517578125
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 548,
			"versionNonce": 1117961225,
			"isDeleted": false,
			"id": "yWWKnSVG",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -343.87527191443814,
			"y": -317.4678999388692,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 219.6669921875,
			"height": 19.5,
			"seed": 867109735,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Abstract analyzer interfaces and logic.",
			"rawText": "Abstract analyzer interfaces and logic.",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Abstract analyzer interfaces and logic.",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 471,
			"versionNonce": 989701383,
			"isDeleted": false,
			"id": "D7j6Y_c9I1cI1N-WfyCC-",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 13.59573385800718,
			"y": -327.6066494697428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 169.54100036621094,
			"height": 46.29690170288086,
			"seed": 1597152679,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 470,
			"versionNonce": 954225385,
			"isDeleted": false,
			"id": "voRcRUrC6DNJhGgVrDIHc",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 163.13674948300718,
			"y": -322.6066494697428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 310268103,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 470,
			"versionNonce": 36793383,
			"isDeleted": false,
			"id": "lbEExIkC14nEbb3jJQz24",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 161.13674948300718,
			"y": -320.6066494697428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1143381991,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 470,
			"versionNonce": 84413897,
			"isDeleted": false,
			"id": "a_qoj5vA-_fq4WXcLkvjT",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 161.13674948300718,
			"y": -316.6066494697428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 909642503,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 468,
			"versionNonce": 966741831,
			"isDeleted": false,
			"id": "eb9xwTAA",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 28.59573385800718,
			"y": -308.6115322822428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 113.6064453125,
			"height": 21,
			"seed": 892277287,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "UnitTestGenerator",
			"rawText": "UnitTestGenerator",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "UnitTestGenerator",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 470,
			"versionNonce": 775253161,
			"isDeleted": false,
			"id": "EQEGkdCiBHiaExpE3XTNF",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 217.86575338925718,
			"y": -324.59667022169594,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 207.6046142578125,
			"height": 40.265594482421875,
			"seed": 1241003335,
			"groupIds": [
				"5F79RE9T-jS8ZdZhHZkNN"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					16.130035400390625
				],
				[
					-34.6099853515625,
					20.130035400390625
				],
				[
					0,
					24.130035400390625
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					172.99462890625,
					40.265594482421875
				],
				[
					172.99462890625,
					40.265594482421875
				],
				[
					172.99462890625,
					40.265594482421875
				],
				[
					172.99462890625,
					10.000030517578125
				],
				[
					162.99462890625,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 470,
			"versionNonce": 195284583,
			"isDeleted": false,
			"id": "H5EayAgjNv1tnAS5d_YXw",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 380.8603822955072,
			"y": -324.59667022169594,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 10,
			"height": 10.000030517578125,
			"seed": 737145959,
			"groupIds": [
				"ZuMqopVvx4KLjZMc-QHqE"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					10.000030517578125
				],
				[
					10,
					10.000030517578125
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 468,
			"versionNonce": 1276374921,
			"isDeleted": false,
			"id": "4N0Tgkkt",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 223.86575338925718,
			"y": -320.5297451728678,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 131.5107421875,
			"height": 19.5,
			"seed": 1927466887,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Abstract test generator",
			"rawText": "Abstract test generator",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Abstract test generator",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 468,
			"versionNonce": 587950471,
			"isDeleted": false,
			"id": "eiMzhTrQ",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 223.86575338925718,
			"y": -305.3969326728678,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 115.62255859375,
			"height": 19.5,
			"seed": 151864999,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "interfaces and logic.",
			"rawText": "interfaces and logic.",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "interfaces and logic.",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 637,
			"versionNonce": 1960777321,
			"isDeleted": false,
			"id": "3PTuVHoIO_TRRlEvqYbI5",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -25.533063283132606,
			"y": -68.5222874788268,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 233.65074801584603,
			"height": 46.29690170288086,
			"seed": 1562106311,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 699,
			"versionNonce": 487760039,
			"isDeleted": false,
			"id": "7qyRqsR5D28ivnLTjET5T",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 187.9218491695632,
			"y": -64.17736485249327,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 1412106471,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 699,
			"versionNonce": 1801609545,
			"isDeleted": false,
			"id": "ewm58VhhmFXDbqF7-pSLc",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 185.9218491695632,
			"y": -62.17736485249327,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 2076660743,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 699,
			"versionNonce": 996537287,
			"isDeleted": false,
			"id": "ugemZxxIEPRNLUK-iib1_",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 185.9218491695632,
			"y": -58.17736485249327,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 2079144743,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 594,
			"versionNonce": 217665577,
			"isDeleted": false,
			"id": "bPYBTPLv",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -10.533063283132606,
			"y": -49.5271702913268,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 188.576171875,
			"height": 21,
			"seed": 1225342535,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "TraceBasedUnitTestGenerator",
			"rawText": "TraceBasedUnitTestGenerator",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "TraceBasedUnitTestGenerator",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 669,
			"versionNonce": 1445061351,
			"isDeleted": false,
			"id": "4XBbBnrhFE3mG4nDJhaa7",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 242.5974596191768,
			"y": -61.49420446205568,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 236.492193820394,
			"height": 40.265594482421875,
			"seed": 368565607,
			"groupIds": [
				"Z9-Tb4XaoeoiH1xxRDLMg"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					16.129974365234375
				],
				[
					-30.262784982555093,
					20.129974365234375
				],
				[
					0,
					24.129974365234375
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					206.2294088378389,
					40.265594482421875
				],
				[
					206.2294088378389,
					40.265594482421875
				],
				[
					206.2294088378389,
					40.265594482421875
				],
				[
					206.2294088378389,
					10
				],
				[
					197.50312910517206,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 701,
			"versionNonce": 2140103433,
			"isDeleted": false,
			"id": "lf2lLYshJjxePEhl-ojU9",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 439.19264880549434,
			"y": -61.49420446205568,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 10,
			"height": 10,
			"seed": 101030023,
			"groupIds": [
				"9QY-J5FBnHWu90Es1y81e"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					10
				],
				[
					10,
					10
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 647,
			"versionNonce": 231927303,
			"isDeleted": false,
			"id": "QguHerbm",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 253.01472834755918,
			"y": -57.42730993080568,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 166.9052734375,
			"height": 19.5,
			"seed": 518975399,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Java-specific implementation",
			"rawText": "Java-specific implementation",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Java-specific implementation",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 627,
			"versionNonce": 530586089,
			"isDeleted": false,
			"id": "clp0Ynfb",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 253.01472834755918,
			"y": -42.2945279483838,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 186.41796875,
			"height": 19.5,
			"seed": 2061525703,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "that generates tests from traces.",
			"rawText": "that generates tests from traces.",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "that generates tests from traces.",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 514,
			"versionNonce": 1031116071,
			"isDeleted": false,
			"id": "K_SwoLvBhZYpAVPqWIZOs",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -566.6342004642007,
			"y": -370.4586276681272,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 918396167,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 470,
			"versionNonce": 775353545,
			"isDeleted": false,
			"id": "uvAvAgOBZT6nc6OHjYPm_",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 417.2799379595697,
			"y": -370.68007476271157,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 2059847271,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 932,
			"versionNonce": 745953351,
			"isDeleted": false,
			"id": "CfAOHVq6lkILUuvvt1ZFw",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -13.123099337244184,
			"y": 91.07774706797085,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 348908487,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 932,
			"versionNonce": 1055880105,
			"isDeleted": false,
			"id": "CsXuTdWVeoztS2tu1uXZL",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -13.180289278650434,
			"y": 91.14622851328335,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1343940103,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 897,
			"versionNonce": 11646823,
			"isDeleted": false,
			"id": "-VsacZzWrSHGzNvj6Qy-I",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -3.8570606504986245,
			"y": 144.91668546918305,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 119558215,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 897,
			"versionNonce": 1710560905,
			"isDeleted": false,
			"id": "zyRWv8bDl7a4U_Yt5bJ7R",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -3.6670582090923745,
			"y": 147.07964933637055,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 855648903,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 482,
			"versionNonce": 1886663303,
			"isDeleted": false,
			"id": "u1UGa4wcVpzlI7dQG63xC",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -513.7168571606301,
			"y": -48.13568747117358,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1407087815,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 483,
			"versionNonce": 637754729,
			"isDeleted": false,
			"id": "WvaGaK0ZoVQajN9xAIb9w",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -82.8585940726681,
			"y": -372.46900622308897,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 819932935,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 525,
			"versionNonce": 880429479,
			"isDeleted": false,
			"id": "cjNBVpXwxh5SK0-liGoSb",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 444.63758597873334,
			"y": -105.5779074644006,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 405166407,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 483,
			"versionNonce": 509986889,
			"isDeleted": false,
			"id": "zY8_9FMZrKnVo7NbpaWjO",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -65.71973909219935,
			"y": -326.9683043187921,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 517469063,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 483,
			"versionNonce": 972061895,
			"isDeleted": false,
			"id": "qzkdze77TWvmZk27d-trJ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -80.4682132132931,
			"y": -372.12119738519834,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1628962247,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "arrow",
			"version": 345,
			"versionNonce": 2029508393,
			"isDeleted": false,
			"id": "4UCu8L5gOj6Ea4PZe6ufx",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -515.9124381337933,
			"y": -497.45312205476534,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 272.4998113281523,
			"height": 125.60227616409975,
			"seed": 1465768041,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "O2BRoRvJxzeTrGsV39kGg",
				"focus": 0.19366000557956137,
				"gap": 1.0000000000001137
			},
			"endBinding": {
				"elementId": "1qA27gcr",
				"focus": -0.14074059568985542,
				"gap": 2.5988222264446676
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-241.35799119120907,
					27.40897129862219
				],
				[
					-272.4998113281523,
					125.60227616409975
				]
			]
		},
		{
			"type": "arrow",
			"version": 297,
			"versionNonce": 343100391,
			"isDeleted": false,
			"id": "_r84BolUeTvk_bHGSi1n9",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -61.287965822053366,
			"y": -496.182949337726,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 263.9729111376289,
			"height": 122.02320329114042,
			"seed": 562444007,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "O2BRoRvJxzeTrGsV39kGg",
				"focus": -0.17004805288916203,
				"gap": 5.624472311739851
			},
			"endBinding": {
				"elementId": "vnPMBuyk",
				"focus": 0.21751798329300567,
				"gap": 4.548183246764609
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					235.71802158200285,
					25.29998091249655
				],
				[
					263.9729111376289,
					122.02320329114042
				]
			]
		},
		{
			"type": "arrow",
			"version": 186,
			"versionNonce": 405080585,
			"isDeleted": false,
			"id": "JuZbzD2cW6JgoA4prZ1Te",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -780.7847405887683,
			"y": -109.31618252875815,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0.5259813079404694,
			"height": 141.13822212980614,
			"seed": 551364711,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "A8qkALfH"
				}
			],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "yhQy6xWL",
				"focus": -0.23631215485768428,
				"gap": 4.171217713834565
			},
			"endBinding": {
				"elementId": "AACdf80OpYKzPzCPtOhfl",
				"focus": 0.0959437443676587,
				"gap": 6.502704760051188
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-0.5259813079404694,
					-141.13822212980614
				]
			]
		},
		{
			"type": "text",
			"version": 66,
			"versionNonce": 806303495,
			"isDeleted": false,
			"id": "A8qkALfH",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -824.5025110552133,
			"y": -203.63965802179692,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 81.8046875,
			"height": 18.4,
			"seed": 684104873,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "implements",
			"rawText": "implements",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "JuZbzD2cW6JgoA4prZ1Te",
			"originalText": "implements",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 344,
			"versionNonce": 82370793,
			"isDeleted": false,
			"id": "zGXnobSRfU3AUZvNbeVgV",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 211.39181081514812,
			"y": -105.85250257836186,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1.5399674125923184,
			"height": 147.8411359922813,
			"seed": 2117530313,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "QzX5exJp"
				}
			],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "pDmPXMl6",
				"focus": 1.0951329134485588,
				"gap": 5.582593977039778
			},
			"endBinding": {
				"elementId": "ed0IN-cIZou1l4l0PGaVp",
				"focus": 0.011841334163884774,
				"gap": 3.623009983572331
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-1.5399674125923184,
					-147.8411359922813
				]
			]
		},
		{
			"type": "text",
			"version": 69,
			"versionNonce": 1128316455,
			"isDeleted": false,
			"id": "QzX5exJp",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -632.1096087047936,
			"y": -110.60065685944208,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 81.8046875,
			"height": 18.4,
			"seed": 1245688233,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "implements",
			"rawText": "implements",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "zGXnobSRfU3AUZvNbeVgV",
			"originalText": "implements",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1133,
			"versionNonce": 483318729,
			"isDeleted": false,
			"id": "24sSnWe7NVy1pYvCvt5oh",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -709.7759775612135,
			"y": 9.289175984159804,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 265.64835582689057,
			"height": 67.84919253150204,
			"seed": 858009191,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "haTpj4rOqeLkWvnsUh7PH",
				"focus": 0.6982286469066157,
				"gap": 2.1292549348255747
			},
			"endBinding": {
				"elementId": "Ay3kuijPCSvrHWLyQuoPB",
				"focus": -0.42897958532573216,
				"gap": 4.614986875396198
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					233.6931120825319,
					21.762144255824957
				],
				[
					265.64835582689057,
					67.84919253150204
				]
			]
		},
		{
			"type": "arrow",
			"version": 1742,
			"versionNonce": 16015687,
			"isDeleted": false,
			"id": "_QVnMz1lfNs0yo7gMSd9N",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 162.60840513544136,
			"y": 10.76093082498943,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 271.2146699350351,
			"height": 67.56453838071741,
			"seed": 1190632681,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "_TI2H9_KkXXRx1-U-5Rb5",
				"focus": -0.7427291560077106,
				"gap": 2.675645540366588
			},
			"endBinding": {
				"elementId": "Ay3kuijPCSvrHWLyQuoPB",
				"focus": 0.3868453009535664,
				"gap": 3.4278861853512126
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-235.18684479985276,
					17.304687644414003
				],
				[
					-271.2146699350351,
					67.56453838071741
				]
			]
		},
		{
			"type": "arrow",
			"version": 301,
			"versionNonce": 2146597545,
			"isDeleted": false,
			"id": "TEkzmNrvhAUC1h4nbJbmc",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -282.6270242431174,
			"y": -446.24350520695344,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0.5002246661279628,
			"height": 70.2113258960303,
			"seed": 1192072807,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "O2BRoRvJxzeTrGsV39kGg",
				"focus": -0.03498957134169751,
				"gap": 4.2138301942139265
			},
			"endBinding": {
				"elementId": "8QYx4syD",
				"focus": 0.04822017782858057,
				"gap": 4.937074455021673
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					0.5002246661279628,
					70.2113258960303
				]
			]
		},
		{
			"type": "rectangle",
			"version": 540,
			"versionNonce": 176595047,
			"isDeleted": false,
			"id": "qkELA9nZ5FvLlfZH19-oa",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4498.923222184217,
			"y": -1908.4074043989206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 533.82421875,
			"height": 416.2578125,
			"seed": 386340583,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "hkGaJ2yxmHKEBJoVs9hl5",
					"type": "arrow"
				},
				{
					"id": "1qlCBd5KNXnxs-J-L_Oc5",
					"type": "arrow"
				}
			],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 456,
			"versionNonce": 1056914825,
			"isDeleted": false,
			"id": "Xreku4w4",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4538.474003434217,
			"y": -1875.4644356489205,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 34.6796875,
			"height": 18.4,
			"seed": 1676780039,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "DiSL",
			"rawText": "DiSL",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "DiSL",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 514,
			"versionNonce": 566492039,
			"isDeleted": false,
			"id": "YrqXDv0C4WFSxmTMvr2bn",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4512.974003434217,
			"y": -1839.4855293989206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 198.94140625,
			"height": 337.05078125,
			"seed": 1167968551,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "3mIOA_nZ-_c3ImYMq9bDp",
					"type": "arrow"
				},
				{
					"id": "14c71GMpxeIi-Bu4Gi604",
					"type": "arrow"
				},
				{
					"id": "HQUyNrHeeu25L0cNTGLUb",
					"type": "arrow"
				},
				{
					"id": "hkGaJ2yxmHKEBJoVs9hl5",
					"type": "arrow"
				}
			],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 503,
			"versionNonce": 1227924585,
			"isDeleted": false,
			"id": "gOuLsmYi",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4522.672548507767,
			"y": -1808.0644356489206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 110.28125,
			"height": 36.8,
			"seed": 1769475143,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Instrumentation\n(DiSLClass)",
			"rawText": "Instrumentation\n(DiSLClass)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Instrumentation\n(DiSLClass)",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 526,
			"versionNonce": 451474087,
			"isDeleted": false,
			"id": "GTsqSNoViuNKIaIsuyrNR",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4802.255253434217,
			"y": -1835.4581856489206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 220,
			"height": 114.01953125,
			"seed": 459074407,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 470,
			"versionNonce": 441593673,
			"isDeleted": false,
			"id": "VAvpe8TI",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4821.474003434217,
			"y": -1812.4644356489205,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 99.6015625,
			"height": 18.4,
			"seed": 2006509191,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Static Context",
			"rawText": "Static Context",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Static Context",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 521,
			"versionNonce": 245499335,
			"isDeleted": false,
			"id": "w_4s7XNExKfo7CzWihfLg",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4808.809940934217,
			"y": -1705.1691231489206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 209.55859375,
			"height": 93.796875,
			"seed": 1558740391,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "14c71GMpxeIi-Bu4Gi604",
					"type": "arrow"
				}
			],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 454,
			"versionNonce": 683630121,
			"isDeleted": false,
			"id": "7gUEcdOf",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4825.474003434217,
			"y": -1689.4644356489205,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 52.46875,
			"height": 18.4,
			"seed": 511193287,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Guards",
			"rawText": "Guards",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Guards",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 482,
			"versionNonce": 1275835623,
			"isDeleted": false,
			"id": "Be86tCUoiCQrcmTrufmUb",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4811.567753434217,
			"y": -1597.8917793989206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 203.6171875,
			"height": 92.89453125,
			"seed": 1977909223,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "HQUyNrHeeu25L0cNTGLUb",
					"type": "arrow"
				}
			],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 455,
			"versionNonce": 325300489,
			"isDeleted": false,
			"id": "bLl4d3Im",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4834.474003434217,
			"y": -1577.4644356489202,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 57.78125,
			"height": 18.4,
			"seed": 892988167,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Markers",
			"rawText": "Markers",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Markers",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 904,
			"versionNonce": 179470343,
			"isDeleted": false,
			"id": "3mIOA_nZ-_c3ImYMq9bDp",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4802.40771127357,
			"y": -1777.5855272269857,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 85.72232484161123,
			"height": 72.2986569909695,
			"seed": 955576871,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": {
				"elementId": "YrqXDv0C4WFSxmTMvr2bn",
				"focus": 0.21230690425614718,
				"gap": 4.769976747742476
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-85.72232484161123,
					72.2986569909695
				]
			]
		},
		{
			"type": "arrow",
			"version": 1178,
			"versionNonce": 263890921,
			"isDeleted": false,
			"id": "14c71GMpxeIi-Bu4Gi604",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4807.19286752357,
			"y": -1660.7394356489206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 89.04163225677303,
			"height": 0.16160683258078734,
			"seed": 436344135,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "w_4s7XNExKfo7CzWihfLg",
				"focus": 0.048326870638853395,
				"gap": 1.6170734106462987
			},
			"endBinding": {
				"elementId": "YrqXDv0C4WFSxmTMvr2bn",
				"focus": 0.05848782919153981,
				"gap": 6.235825582581128
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-89.04163225677303,
					-0.16160683258078734
				]
			]
		},
		{
			"type": "arrow",
			"version": 1171,
			"versionNonce": 1553131303,
			"isDeleted": false,
			"id": "HQUyNrHeeu25L0cNTGLUb",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4810.468760690989,
			"y": -1549.4152168989208,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 92.03873109161123,
			"height": 61.37745566741921,
			"seed": 532534375,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "Be86tCUoiCQrcmTrufmUb",
				"focus": -0.6179366416684149,
				"gap": 1.0989927432274271
			},
			"endBinding": {
				"elementId": "YrqXDv0C4WFSxmTMvr2bn",
				"focus": -0.04475234888790825,
				"gap": 6.514619915161347
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-92.03873109161123,
					-61.37745566741921
				]
			]
		},
		{
			"type": "rectangle",
			"version": 676,
			"versionNonce": 399491785,
			"isDeleted": false,
			"id": "XXxHpN-BCCOllucES6LmM",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4098.907597184217,
			"y": -1767.5128731489206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 222.59765625,
			"height": 127.49609375,
			"seed": 1911861127,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "hkGaJ2yxmHKEBJoVs9hl5",
					"type": "arrow"
				}
			],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 526,
			"versionNonce": 1612278343,
			"isDeleted": false,
			"id": "xNhDIm9D",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4136.415409684217,
			"y": -1729.3027168989206,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 77.3828125,
			"height": 36.8,
			"seed": 182658727,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Target App\n(JAR)",
			"rawText": "Target App\n(JAR)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Target App\n(JAR)",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1552,
			"versionNonce": 1480491433,
			"isDeleted": false,
			"id": "hkGaJ2yxmHKEBJoVs9hl5",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4321.132595012282,
			"y": -1704.0829805302913,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 172.59498326354577,
			"height": 1.2291889674399954,
			"seed": 1765128647,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": {
				"elementId": "YrqXDv0C4WFSxmTMvr2bn",
				"focus": 0.18346318139346937,
				"gap": 19.246425158388774
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					172.59498326354577,
					1.2291889674399954
				]
			]
		},
		{
			"type": "text",
			"version": 503,
			"versionNonce": 841124199,
			"isDeleted": false,
			"id": "n8YMpQbv",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4542.184940934217,
			"y": -1675.1675606489205,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 98.390625,
			"height": 18.4,
			"seed": 34138343,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "GENERATED",
			"rawText": "GENERATED",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "GENERATED",
			"lineHeight": 1.15
		},
		{
			"type": "text",
			"version": 606,
			"versionNonce": 1580360841,
			"isDeleted": false,
			"id": "hEJBZ9do",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4330.157483094863,
			"y": -1784.293787396663,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 91.6171875,
			"height": 55.199999999999996,
			"seed": 1927885831,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Arguments\nfor launching\nDiSL",
			"rawText": "Arguments\nfor launching\nDiSL",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Arguments\nfor launching\nDiSL",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1164,
			"versionNonce": 536528007,
			"isDeleted": false,
			"id": "1qlCBd5KNXnxs-J-L_Oc5",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 5051.67222697354,
			"y": -1691.268201814729,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 194.7292726091879,
			"height": 1.7367336836232425,
			"seed": 1943214887,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "qkELA9nZ5FvLlfZH19-oa",
				"focus": 0.030692214927164098,
				"gap": 18.924786039323408
			},
			"endBinding": {
				"elementId": "-zwjVs6fHLtTU44Lc__YU",
				"focus": -0.006671710718883719,
				"gap": 4.533034247621799
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					194.7292726091879,
					1.7367336836232425
				]
			]
		},
		{
			"type": "rectangle",
			"version": 512,
			"versionNonce": 90631017,
			"isDeleted": false,
			"id": "-zwjVs6fHLtTU44Lc__YU",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 5250.93453383035,
			"y": -1739.3151781722138,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 132.84419346063078,
			"height": 100.15695353445597,
			"seed": 154345031,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "1qlCBd5KNXnxs-J-L_Oc5",
					"type": "arrow"
				}
			],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 475,
			"versionNonce": 425630631,
			"isDeleted": false,
			"id": "CquY9FUs",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 5270.078437691574,
			"y": -1699.9538207173061,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 69.3671875,
			"height": 18.4,
			"seed": 1977990503,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747765,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Exit Code",
			"rawText": "Exit Code",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Exit Code",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1041,
			"versionNonce": 640128585,
			"isDeleted": false,
			"id": "KYxrWNbapw_EaAL-yW6GW",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 5131.632003964778,
			"y": -1688.660306360712,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 122.48241891225348,
			"height": 132.47208124615088,
			"seed": 1832351879,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": {
				"elementId": "lYvrkIhM",
				"focus": 0.914292463306913,
				"gap": 17.964836956745785
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					16.177992399237155,
					118.40579562046469
				],
				[
					122.48241891225348,
					132.47208124615088
				]
			]
		},
		{
			"type": "rectangle",
			"version": 681,
			"versionNonce": 1652199111,
			"isDeleted": false,
			"id": "PEl_FwYlfr5R7BNuR-JaW",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 5258.1682686226895,
			"y": -1590.8085148858802,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 238.14019804269861,
			"height": 202.64814087304774,
			"seed": 82104231,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "KYxrWNbapw_EaAL-yW6GW",
					"type": "arrow"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 596,
			"versionNonce": 55567657,
			"isDeleted": false,
			"id": "lYvrkIhM",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 5272.0792598337775,
			"y": -1543.4499901209324,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 119.1640625,
			"height": 110.39999999999999,
			"seed": 1729774279,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "KYxrWNbapw_EaAL-yW6GW",
					"type": "arrow"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Instrumentation \ncode execution\n\n(prints to stdout, \nstderr, writing to \nsockets, etc.)",
			"rawText": "Instrumentation \ncode execution\n\n(prints to stdout, \nstderr, writing to \nsockets, etc.)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Instrumentation \ncode execution\n\n(prints to stdout, \nstderr, writing to \nsockets, etc.)",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 699,
			"versionNonce": 99294695,
			"isDeleted": false,
			"id": "aJanlt5gI-fuc5saRgQXt",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4053.6484590649206,
			"y": -1945.0792181136405,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1465.255123166433,
			"height": 592.2737437394762,
			"seed": 1148473831,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 545,
			"versionNonce": 2018203657,
			"isDeleted": false,
			"id": "f1n31kf4",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4083.820679264175,
			"y": -1920.6657736569193,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 104.625,
			"height": 18.4,
			"seed": 100629767,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "DiSL Workflow",
			"rawText": "DiSL Workflow",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "DiSL Workflow",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 439,
			"versionNonce": 113035527,
			"isDeleted": false,
			"id": "4plA4z22qN1QIwUAc58Mg",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1300.4283663261676,
			"y": -959.7703564456472,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 222.62865463844707,
			"height": 126.21857423162749,
			"seed": 1060652071,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "CvRWN7el"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 394,
			"versionNonce": 1729457897,
			"isDeleted": false,
			"id": "CvRWN7el",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1370.8286311453912,
			"y": -915.0610693298335,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 81.828125,
			"height": 36.8,
			"seed": 1226942439,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Target App \n(JAR)",
			"rawText": "Target App \n(JAR)",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "4plA4z22qN1QIwUAc58Mg",
			"originalText": "Target App \n(JAR)",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 546,
			"versionNonce": 1382813735,
			"isDeleted": false,
			"id": "jpVAasQuQpt8Bb2Fv2Pa1",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1304.6010583249786,
			"y": -808.3644065032925,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 212.7214837990961,
			"height": 107.79593346099921,
			"seed": 1611343687,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "Pj2R8O1q"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 438,
			"versionNonce": 1715055049,
			"isDeleted": false,
			"id": "Pj2R8O1q",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1326.9149252245265,
			"y": -763.6664397727928,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 168.09375,
			"height": 18.4,
			"seed": 1829247527,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Target Method Identifier",
			"rawText": "Target Method Identifier",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "jpVAasQuQpt8Bb2Fv2Pa1",
			"originalText": "Target Method Identifier",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 432,
			"versionNonce": 1927639879,
			"isDeleted": false,
			"id": "Y-i82hdX8vy4gJffDCC0v",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1301.45233502046,
			"y": -678.1359940302802,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 215.15696038822523,
			"height": 108.29172690950054,
			"seed": 651470439,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "CNetsXkr"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 390,
			"versionNonce": 785615017,
			"isDeleted": false,
			"id": "CNetsXkr",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1323.6480027145726,
			"y": -633.19013057553,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 170.765625,
			"height": 18.4,
			"seed": 1999043687,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Level of Instrumentation",
			"rawText": "Level of Instrumentation",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "Y-i82hdX8vy4gJffDCC0v",
			"originalText": "Level of Instrumentation",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 516,
			"versionNonce": 8745575,
			"isDeleted": false,
			"id": "1xgIPRtk2o3RShlPXnVR2",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1285.35209514018,
			"y": -1042.4398011106507,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 254.75085122292774,
			"height": 487.16490287126965,
			"seed": 1382105479,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 370,
			"versionNonce": 1250855817,
			"isDeleted": false,
			"id": "aWeG3tZi",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1317.4308010712832,
			"y": -1009.4963437465342,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 35.5859375,
			"height": 18.4,
			"seed": 334330023,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Input",
			"rawText": "Input",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Input",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 428,
			"versionNonce": 1410895239,
			"isDeleted": false,
			"id": "_mtGEPTKBjeqdHOSfsVM1",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1777.8813420301663,
			"y": -1160.8619277252676,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1985.5483836797116,
			"height": 750.6051866389836,
			"seed": 1092996039,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 340,
			"versionNonce": 27669097,
			"isDeleted": false,
			"id": "hhy4wMYkJ8l4SWhVJ90pd",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1828.4522737773013,
			"y": -1006.9832979456717,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 282.91539835007336,
			"height": 513.1288229375232,
			"seed": 1467794951,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 262,
			"versionNonce": 2027635879,
			"isDeleted": false,
			"id": "es7ralVr",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1858.2955601248113,
			"y": -966.4115799791755,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 46.25,
			"height": 18.4,
			"seed": 1680108839,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "runner",
			"rawText": "runner",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "runner",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 285,
			"versionNonce": 149290313,
			"isDeleted": false,
			"id": "0dnYqKgvxeFVT_EV8V-zq",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1853.3724181624998,
			"y": -924.1336032618976,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 234.379829180996,
			"height": 119.59059865692552,
			"seed": 1200238663,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 266,
			"versionNonce": 1585059783,
			"isDeleted": false,
			"id": "igbUBQ7y",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1878.3360532010756,
			"y": -886.2496076741184,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 116.4921875,
			"height": 18.4,
			"seed": 998936423,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "ArgumentParser",
			"rawText": "ArgumentParser",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "ArgumentParser",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 355,
			"versionNonce": 1873374249,
			"isDeleted": false,
			"id": "DHmVQBBi66K-fYQmO1ObH",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2204.7325765683604,
			"y": -1122.250925656893,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 446.30978308862495,
			"height": 186.98371513040422,
			"seed": 175245959,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 357,
			"versionNonce": 1259439847,
			"isDeleted": false,
			"id": "XVL5dYm1",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2221.9896678284767,
			"y": -1095.6744999472141,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 155.6015625,
			"height": 18.4,
			"seed": 16589223,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "instrumentor-common",
			"rawText": "instrumentor-common",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "instrumentor-common",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 665,
			"versionNonce": 1306263305,
			"isDeleted": false,
			"id": "3jv-HlwAKN4zHOFD9U8hS",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3123.2152141778156,
			"y": -1788.5056222197275,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 446.30978308862495,
			"height": 186.98371513040422,
			"seed": 542659783,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 599,
			"versionNonce": 221123079,
			"isDeleted": false,
			"id": "k5Cp1Z2K",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3155.028627123317,
			"y": -1752.1481485654915,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 123.6015625,
			"height": 18.4,
			"seed": 270541799,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "instrumentor-java",
			"rawText": "instrumentor-java",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "instrumentor-java",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 794,
			"versionNonce": 838415849,
			"isDeleted": false,
			"id": "Cl-iWp2DhD8-TIcxd0biY",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3130.660814036011,
			"y": -1386.7737588428447,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 434.90653377309445,
			"height": 55.75501762971089,
			"seed": 1348538119,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 661,
			"versionNonce": 1842100519,
			"isDeleted": false,
			"id": "qFbGf4DL",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3145.739023561924,
			"y": -1367.2297717842769,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 100.4765625,
			"height": 18.4,
			"seed": 731354663,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "instrumentor-*",
			"rawText": "instrumentor-*",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "instrumentor-*",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 671,
			"versionNonce": 1326223561,
			"isDeleted": false,
			"id": "ZXXHYllBiFobz4tIC3-Nf",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3133.0397527757505,
			"y": -1576.466941678655,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 428.91352173770133,
			"height": 52.58889806384252,
			"seed": 1756379463,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "MF52ARC8"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 565,
			"versionNonce": 1649966151,
			"isDeleted": false,
			"id": "MF52ARC8",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 3340.8285448946012,
			"y": -1559.372492646734,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 1928187495,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "ZXXHYllBiFobz4tIC3-Nf",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 707,
			"versionNonce": 1790021545,
			"isDeleted": false,
			"id": "xDxEAZbyiQYdyGUfjpH0y",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3131.4262495354524,
			"y": -1507.7865018652078,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 428.91352173770133,
			"height": 52.58889806384252,
			"seed": 1023899751,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "v2Z2YevL"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 564,
			"versionNonce": 2125421415,
			"isDeleted": false,
			"id": "v2Z2YevL",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 3339.215041654303,
			"y": -1490.6920528332867,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 970760359,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "xDxEAZbyiQYdyGUfjpH0y",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 755,
			"versionNonce": 185840265,
			"isDeleted": false,
			"id": "InfAUdWdCkBPEsxVKNFCK",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3132.4178364324544,
			"y": -1446.673435739413,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 428.91352173770133,
			"height": 52.58889806384252,
			"seed": 2120864647,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "9E1ebEIT"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 563,
			"versionNonce": 656234119,
			"isDeleted": false,
			"id": "9E1ebEIT",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 3340.206628551305,
			"y": -1429.5789867074918,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 1215029991,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "InfAUdWdCkBPEsxVKNFCK",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 774,
			"versionNonce": 1693350249,
			"isDeleted": false,
			"id": "mTE8ak6jbLfUN78xy-dpc",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3097.2904356996014,
			"y": -1811.2642811320732,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 503.17816144479474,
			"height": 490.1570598236284,
			"seed": 1820148295,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "rectangle",
			"version": 664,
			"versionNonce": 39775655,
			"isDeleted": false,
			"id": "5GNfcI4RMil-XqPADQUK2",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2513.4753259958893,
			"y": -1968.6460566203427,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 349.5430793241112,
			"height": 620.3419816432666,
			"seed": 422671271,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 568,
			"versionNonce": 160261193,
			"isDeleted": false,
			"id": "Y0UCHtcc",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2536.429692848433,
			"y": -1940.5126655197569,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 89.8125,
			"height": 18.4,
			"seed": 305292999,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "analyzer-disl",
			"rawText": "analyzer-disl",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "analyzer-disl",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 572,
			"versionNonce": 731947207,
			"isDeleted": false,
			"id": "2DwlJf13poW0aVfgNrsGO",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2538.6477161706757,
			"y": -1648.4330739338889,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 293.27487198454855,
			"height": 60.49549884783744,
			"seed": 552072679,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "x1gzOHdi"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 526,
			"versionNonce": 1422652201,
			"isDeleted": false,
			"id": "x1gzOHdi",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 2647.94140216295,
			"y": -1627.3853245099701,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 74.6875,
			"height": 18.4,
			"seed": 500211815,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "DiSLClass",
			"rawText": "DiSLClass",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "2DwlJf13poW0aVfgNrsGO",
			"originalText": "DiSLClass",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 658,
			"versionNonce": 612298727,
			"isDeleted": false,
			"id": "B2_1JTV5naAx6BBkYNDcv",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2541.1179852825067,
			"y": -1563.6610923708372,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 291.72660472431653,
			"height": 47.29173648248627,
			"seed": 1549705479,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "MvSzbIdZ"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 571,
			"versionNonce": 454403593,
			"isDeleted": false,
			"id": "MvSzbIdZ",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 2637.180506394665,
			"y": -1549.2152241295942,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 99.6015625,
			"height": 18.4,
			"seed": 1672671911,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Static Context",
			"rawText": "Static Context",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "B2_1JTV5naAx6BBkYNDcv",
			"originalText": "Static Context",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 572,
			"versionNonce": 117857031,
			"isDeleted": false,
			"id": "IDI0Z619QwPr-iy8cjRR0",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2540.4569273511725,
			"y": -1494.7110105064507,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 294.84053550613135,
			"height": 48.32681403286631,
			"seed": 1175644199,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "SWptRktW"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 488,
			"versionNonce": 392559849,
			"isDeleted": false,
			"id": "SWptRktW",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 2661.642820104238,
			"y": -1479.7476034900176,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 52.46875,
			"height": 18.4,
			"seed": 1784001767,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Guards",
			"rawText": "Guards",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "IDI0Z619QwPr-iy8cjRR0",
			"originalText": "Guards",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 574,
			"versionNonce": 2008244775,
			"isDeleted": false,
			"id": "ur42lOKKsEKsjvHRvQoDU",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2538.4737535571667,
			"y": -1423.8647361548142,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 291.343886974596,
			"height": 50.44045978700353,
			"seed": 2079411015,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "4lDE5qH3"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 489,
			"versionNonce": 424179657,
			"isDeleted": false,
			"id": "4lDE5qH3",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 2655.2550720444647,
			"y": -1407.8445062613125,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 57.78125,
			"height": 18.4,
			"seed": 1885136679,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Markers",
			"rawText": "Markers",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "ur42lOKKsEKsjvHRvQoDU",
			"originalText": "Markers",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 554,
			"versionNonce": 1467123015,
			"isDeleted": false,
			"id": "03idwWQZtiawCLLaYJBpc",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2530.636737818577,
			"y": -1671.1177987354945,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 314.2982538271399,
			"height": 306.29597360571483,
			"seed": 710298215,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "KWLedpi5ZjaYm-NtqaS8u",
					"type": "arrow"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "rectangle",
			"version": 609,
			"versionNonce": 1926796969,
			"isDeleted": false,
			"id": "vb8O5anGo4iojAsyFS0vd",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2535.7425405250733,
			"y": -1895.1642486740411,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 285.994536609187,
			"height": 62.25252124428073,
			"seed": 1448484007,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "HBDpc5do"
				},
				{
					"id": "KWLedpi5ZjaYm-NtqaS8u",
					"type": "arrow"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 526,
			"versionNonce": 899695719,
			"isDeleted": false,
			"id": "HBDpc5do",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 2624.497621329667,
			"y": -1873.2379880519009,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 108.484375,
			"height": 18.4,
			"seed": 2015927207,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "ResultCollector",
			"rawText": "ResultCollector",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "vb8O5anGo4iojAsyFS0vd",
			"originalText": "ResultCollector",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1404,
			"versionNonce": 650238345,
			"isDeleted": false,
			"id": "KWLedpi5ZjaYm-NtqaS8u",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2678.0572123913184,
			"y": -1678.2502658893743,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4.082863354622077,
			"height": 153.02621297339988,
			"seed": 1993994183,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "03idwWQZtiawCLLaYJBpc",
				"gap": 7.132467153879475,
				"focus": -0.08686106575845451
			},
			"endBinding": {
				"elementId": "vb8O5anGo4iojAsyFS0vd",
				"gap": 1.6352485669862062,
				"focus": -0.02971867918485993
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					4.082863354622077,
					-153.02621297339988
				]
			]
		},
		{
			"type": "rectangle",
			"version": 325,
			"versionNonce": 34152327,
			"isDeleted": false,
			"id": "Fz2D6t6Ej1YtDdER6LXc9",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3367.7256668910914,
			"y": -1114.2138529127662,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 356.2493380748929,
			"height": 158.76697921920584,
			"seed": 2041994983,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 297,
			"versionNonce": 1447018601,
			"isDeleted": false,
			"id": "aZ7QBtj1",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3395.907610279588,
			"y": -1084.271250631684,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 166.2890625,
			"height": 18.4,
			"seed": 1098377735,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "test-generator-common",
			"rawText": "test-generator-common",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "test-generator-common",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 462,
			"versionNonce": 2129237671,
			"isDeleted": false,
			"id": "KKdgxtcUIc9YPwJpF47Vw",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3377.4327807249065,
			"y": -857.4537335038069,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 349.13426718236497,
			"height": 139.00482632455646,
			"seed": 723194151,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "1OOJlzVwTZg_wBKSOZS-A",
					"type": "arrow"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 361,
			"versionNonce": 1864223561,
			"isDeleted": false,
			"id": "7oQWMrp0",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3399.2737868509917,
			"y": -833.5650301728467,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 134.2890625,
			"height": 18.4,
			"seed": 1074549831,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "test-generator-java",
			"rawText": "test-generator-java",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "test-generator-java",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 808,
			"versionNonce": 1928210887,
			"isDeleted": false,
			"id": "O94vTlbtY26bsdhuWq3KA",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3372.7984639145684,
			"y": -505.1207779573044,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 350.61562213639456,
			"height": 53.180370949774215,
			"seed": 673723239,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 661,
			"versionNonce": 431454761,
			"isDeleted": false,
			"id": "BOHUxaAg",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3392.3750470560817,
			"y": -486.3957055857338,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 111.1640625,
			"height": 18.4,
			"seed": 1538993799,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "test-generator-*",
			"rawText": "test-generator-*",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "test-generator-*",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 704,
			"versionNonce": 456349927,
			"isDeleted": false,
			"id": "F4QrLfSLZeUWV7F277iBX",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3377.2718653074107,
			"y": -693.0003513555178,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 349.99538211923556,
			"height": 51.48423546805884,
			"seed": 360826279,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "Xeq70799"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 749,
			"versionNonce": 1727548681,
			"isDeleted": false,
			"id": "Xeq70799",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 3545.6015876170286,
			"y": -676.4582336214884,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 704911303,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "F4QrLfSLZeUWV7F277iBX",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 689,
			"versionNonce": 1564845063,
			"isDeleted": false,
			"id": "7ex94JhNh1mjwWPJH8Ztj",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3375.3104368400946,
			"y": -627.0511245741657,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 352.8222745887609,
			"height": 47.2308495677579,
			"seed": 78149831,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "C33mBKb8"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 696,
			"versionNonce": 1046201321,
			"isDeleted": false,
			"id": "C33mBKb8",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 3545.053605384475,
			"y": -612.6356997902867,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 2117603847,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "7ex94JhNh1mjwWPJH8Ztj",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 761,
			"versionNonce": 958499623,
			"isDeleted": false,
			"id": "WZIh3da1BaJyw1ZREShdJ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3374.5884919940313,
			"y": -565.9380584483708,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 348.0817933706339,
			"height": 45.369449603209326,
			"seed": 1053542375,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "7KzAcbwK"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 718,
			"versionNonce": 200105673,
			"isDeleted": false,
			"id": "7KzAcbwK",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 3541.9614199293483,
			"y": -552.4533336467662,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 417496135,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "WZIh3da1BaJyw1ZREShdJ",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 346,
			"versionNonce": 2066976327,
			"isDeleted": false,
			"id": "6YRqWx0wLX1iQYVOX9J_j",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3363.8289043484847,
			"y": -875.893770535786,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 376.8030208610089,
			"height": 439.33388228690444,
			"seed": 1912091399,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "rectangle",
			"version": 650,
			"versionNonce": 776510889,
			"isDeleted": false,
			"id": "Vfy61zVDEFHDJff8RJs4A",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4101.065064139289,
			"y": -1226.0080968013774,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1041.7664128693946,
			"height": 811.7850081270759,
			"seed": 918625383,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 391,
			"versionNonce": 777530727,
			"isDeleted": false,
			"id": "RqXikZfq7jEITvQRAmGwI",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4141.493975518835,
			"y": -1127.9134087266268,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 342.2366495567238,
			"height": 163.7944987496228,
			"seed": 1498347399,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "0wig72cHMDqYUvG979S1_",
					"type": "arrow"
				}
			],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 267,
			"versionNonce": 614006921,
			"isDeleted": false,
			"id": "vxR8iO4V",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4162.952263895199,
			"y": -1097.7881457013602,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 144.046875,
			"height": 18.4,
			"seed": 531918503,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747766,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "test-runner-common",
			"rawText": "test-runner-common",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "test-runner-common",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 519,
			"versionNonce": 1306257543,
			"isDeleted": false,
			"id": "CSRhCVDJAdGoksmxus70X",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4156.367778973876,
			"y": -866.4432515569044,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 323.5530648658312,
			"height": 163.7944987496228,
			"seed": 1688076743,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "1OOJlzVwTZg_wBKSOZS-A",
					"type": "arrow"
				},
				{
					"id": "X9mExnt6DTtLoUNTqkrsQ",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 384,
			"versionNonce": 2090469225,
			"isDeleted": false,
			"id": "DrBMTT0b",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4177.130216896202,
			"y": -837.7140385050491,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 112.046875,
			"height": 18.4,
			"seed": 2032677095,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "test-runner-java",
			"rawText": "test-runner-java",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "test-runner-java",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 480,
			"versionNonce": 930824103,
			"isDeleted": false,
			"id": "hz3IxPfMbYZgz6DBL4qXi",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4130.46474582235,
			"y": -882.16947181814,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 376.8030208610089,
			"height": 447.1621998948202,
			"seed": 1457486855,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "0wig72cHMDqYUvG979S1_",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "rectangle",
			"version": 880,
			"versionNonce": 9908809,
			"isDeleted": false,
			"id": "Prf9-YsApFXYWSmFH8Jm6",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4146.12138103818,
			"y": -500.7542671664671,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 348.3688316829245,
			"height": 52.06701022331504,
			"seed": 297820967,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 803,
			"versionNonce": 1566749383,
			"isDeleted": false,
			"id": "mU6LdEK-EhY61k6YtmBnp",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4150.944494497726,
			"y": -683.42805854718,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 341.7495542388977,
			"height": 51.48423546805884,
			"seed": 171402823,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "h0Uqs2oD"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 836,
			"versionNonce": 627835177,
			"isDeleted": false,
			"id": "h0Uqs2oD",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 4315.151302867174,
			"y": -666.8859408131506,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 1786259911,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "mU6LdEK-EhY61k6YtmBnp",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 806,
			"versionNonce": 942588391,
			"isDeleted": false,
			"id": "wfuSe9bbDox6PSlyKiB_F",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4148.983066030408,
			"y": -617.4788317658279,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 340.50572155230685,
			"height": 46.630678551150936,
			"seed": 1595999591,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "sDigU35S"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 782,
			"versionNonce": 948171785,
			"isDeleted": false,
			"id": "sDigU35S",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 4312.567958056561,
			"y": -603.3634924902524,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 2052039687,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "wfuSe9bbDox6PSlyKiB_F",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 877,
			"versionNonce": 2028727559,
			"isDeleted": false,
			"id": "nzT1J9NQ1kU5fT8m1hc4D",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4148.261121184345,
			"y": -556.365765640033,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 341.7843467615994,
			"height": 44.79537297862897,
			"seed": 533315719,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "46xMP4n1"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 837,
			"versionNonce": 1306168041,
			"isDeleted": false,
			"id": "46xMP4n1",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 4312.485325815145,
			"y": -543.1680791507185,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.3359375,
			"height": 18.4,
			"seed": 1572703815,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "...",
			"rawText": "...",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "nzT1J9NQ1kU5fT8m1hc4D",
			"originalText": "...",
			"lineHeight": 1.15
		},
		{
			"type": "text",
			"version": 309,
			"versionNonce": 2065085479,
			"isDeleted": false,
			"id": "M97Zfzpi",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4173.514722564076,
			"y": -484.69561749715035,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 88.921875,
			"height": 18.4,
			"seed": 1039013799,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "test-runner-*",
			"rawText": "test-runner-*",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "test-runner-*",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 790,
			"versionNonce": 1845183945,
			"isDeleted": false,
			"id": "0wig72cHMDqYUvG979S1_",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4511.121038572589,
			"y": -673.1664388828051,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 103.15113168030257,
			"height": 384.98796182661863,
			"seed": 835426791,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "hz3IxPfMbYZgz6DBL4qXi",
				"focus": 0.7848021687588911,
				"gap": 3.8532718892302
			},
			"endBinding": {
				"elementId": "RqXikZfq7jEITvQRAmGwI",
				"focus": -0.5819709078409983,
				"gap": 2.6355335946645937
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					78.39625177793778,
					-335.58257958999513
				],
				[
					-24.75487990236479,
					-384.98796182661863
				]
			]
		},
		{
			"type": "arrow",
			"version": 662,
			"versionNonce": 1989908295,
			"isDeleted": false,
			"id": "1OOJlzVwTZg_wBKSOZS-A",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3739.779508403303,
			"y": -787.416385304997,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 412.91765942552684,
			"height": 0.608869147282121,
			"seed": 1496171783,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "KKdgxtcUIc9YPwJpF47Vw",
				"focus": 0.003699034591798157,
				"gap": 13.212460496031781
			},
			"endBinding": {
				"elementId": "CSRhCVDJAdGoksmxus70X",
				"focus": 0.024563629807367936,
				"gap": 3.6706111450457684
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					412.91765942552684,
					0.608869147282121
				]
			]
		},
		{
			"type": "rectangle",
			"version": 360,
			"versionNonce": 67216553,
			"isDeleted": false,
			"id": "hsn9qFJEZIeDe30Hm3-fq",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4606.8946658742,
			"y": -1134.4840547910592,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 509.5418828592856,
			"height": 695.660081295291,
			"seed": 626887719,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 250,
			"versionNonce": 1909921383,
			"isDeleted": false,
			"id": "oxjul6ug",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4632.998956014789,
			"y": -1105.6564536138678,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 16,
			"height": 18.4,
			"seed": 1648848711,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "UI",
			"rawText": "UI",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "UI",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 314,
			"versionNonce": 194586505,
			"isDeleted": false,
			"id": "80NMSwnVT5QDzp__iRIur",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4633.2145666708175,
			"y": -1039.5691638550963,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 463.6630154086556,
			"height": 116.41435349422977,
			"seed": 709357159,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "X9mExnt6DTtLoUNTqkrsQ",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 263,
			"versionNonce": 1939190151,
			"isDeleted": false,
			"id": "WZGOceJK",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4662.568417412979,
			"y": -1016.9480694192964,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 109.390625,
			"height": 18.4,
			"seed": 405506439,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "TraceVisualizer",
			"rawText": "TraceVisualizer",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "TraceVisualizer",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 707,
			"versionNonce": 2130529897,
			"isDeleted": false,
			"id": "X9mExnt6DTtLoUNTqkrsQ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4488.416610386555,
			"y": -792.6949627024962,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 140.79375838659143,
			"height": 142.9498649468767,
			"seed": 959365287,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "CSRhCVDJAdGoksmxus70X",
				"focus": 0.6692263371865623,
				"gap": 8.495766546847335
			},
			"endBinding": {
				"elementId": "80NMSwnVT5QDzp__iRIur",
				"focus": 0.659868593092884,
				"gap": 4.00419789767102
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					140.79375838659143,
					-142.9498649468767
				]
			]
		},
		{
			"type": "text",
			"version": 278,
			"versionNonce": 1964239015,
			"isDeleted": false,
			"id": "NwpYt9oG",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 4153.973681364105,
			"y": -1192.3935403818925,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 102.8671875,
			"height": 18.4,
			"seed": 1056838599,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Test Visualizer",
			"rawText": "Test Visualizer",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Test Visualizer",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 884,
			"versionNonce": 83466569,
			"isDeleted": false,
			"id": "ZiqkFJghy4JGhbBGVwSgQ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3166.8331473003227,
			"y": -1717.889131883648,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 159.93836340160425,
			"height": 98.6296906817633,
			"seed": 1642270215,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "xNPxWR71"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 692,
			"versionNonce": 335428551,
			"isDeleted": false,
			"id": "xNPxWR71",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 3197.8882665011247,
			"y": -1677.7742865427663,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 97.828125,
			"height": 18.4,
			"seed": 649800423,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "InstGenerator",
			"rawText": "InstGenerator",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "ZiqkFJghy4JGhbBGVwSgQ",
			"originalText": "InstGenerator",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 814,
			"versionNonce": 1592363049,
			"isDeleted": false,
			"id": "ZHHOA37eYrew5LQ6dHDqR",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3371.47436740881,
			"y": -1717.3975954558848,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 156.8253264263749,
			"height": 97.33665114155428,
			"seed": 1518449959,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"type": "text",
					"id": "4b7T4Yfi"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"customData": {
				"legacyTextWrap": true
			}
		},
		{
			"type": "text",
			"version": 679,
			"versionNonce": 74525415,
			"isDeleted": false,
			"id": "4b7T4Yfi",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 3404.9846868719974,
			"y": -1677.9292698851077,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 89.8046875,
			"height": 18.4,
			"seed": 509402407,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "InstCompiler",
			"rawText": "InstCompiler",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "ZHHOA37eYrew5LQ6dHDqR",
			"originalText": "InstCompiler",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 667,
			"versionNonce": 381197065,
			"isDeleted": false,
			"id": "FXGbQ3EFByZ0queYXWkKA",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1499.5273953237356,
			"y": -951.883943512674,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 1778644457,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 667,
			"versionNonce": 1123628551,
			"isDeleted": false,
			"id": "LFk5b0pfYvCc-IIlJC8UO",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1497.5273953237356,
			"y": -949.883943512674,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 935284937,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 667,
			"versionNonce": 1826771433,
			"isDeleted": false,
			"id": "d6q9Z--bG4r5CT3QqsVTl",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1497.5273953237356,
			"y": -945.883943512674,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 789335977,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "line",
			"version": 1297,
			"versionNonce": 110288167,
			"isDeleted": false,
			"id": "xKBRVWGk_CAWG9xglIPPP",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 41.053478894281625,
			"y": -48.88062391322842,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1861839655,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "arrow",
			"version": 277,
			"versionNonce": 108145865,
			"isDeleted": false,
			"id": "jmbEZGJVbPLZ2KtJrb5_I",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -278.75058374466005,
			"y": -111.23985942873938,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0.2955221576758049,
			"height": 143.00957314855546,
			"seed": 1986634727,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "CXLKzc7A"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "Xov1aFklQprShFi4Pr4ix",
				"focus": -0.018598648754454786,
				"gap": 5.995194576959165
			},
			"endBinding": {
				"elementId": "6iq3uEk34t_ASRCYzQEzX",
				"focus": -0.04735458953067691,
				"gap": 4.550758033001159
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					-0.2955221576758049,
					-143.00957314855546
				]
			]
		},
		{
			"type": "text",
			"version": 69,
			"versionNonce": 1923784775,
			"isDeleted": false,
			"id": "CXLKzc7A",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -413.85690197939124,
			"y": -191.46515030447242,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 81.8046875,
			"height": 18.4,
			"seed": 830115591,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "implements",
			"rawText": "implements",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "jmbEZGJVbPLZ2KtJrb5_I",
			"originalText": "implements",
			"lineHeight": 1.15
		},
		{
			"type": "ellipse",
			"version": 698,
			"versionNonce": 1526967209,
			"isDeleted": false,
			"id": "e0lxewGjD18Ayu__mobQK",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -961.8213139840609,
			"y": 868.3842922321207,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 16,
			"height": 16,
			"seed": 1334049225,
			"groupIds": [
				"9j1WwVH9ck-3Z_8cKHUtV",
				"6KnGrFxAxvHy4OfAU3AdQ"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "qdiiIkeKH917XjqwzjG8m",
					"type": "arrow"
				},
				{
					"id": "gt_8of0EwYJZfwTcFxesQ",
					"type": "arrow"
				},
				{
					"id": "b-hdk3tSRfh4yX3dLj3-6",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "line",
			"version": 697,
			"versionNonce": 152895335,
			"isDeleted": false,
			"id": "UpwiNZGydednIFiOdjFLF",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -953.8213139840609,
			"y": 884.3842922321207,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 27.000015258789062,
			"seed": 441702569,
			"groupIds": [
				"x0LvFsCJmw6ka235WQ6IF"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					27.000015258789062
				]
			]
		},
		{
			"type": "line",
			"version": 697,
			"versionNonce": 1656901257,
			"isDeleted": false,
			"id": "WpQ0lI8V9flCOYFG2QIL_",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -966.8213130303866,
			"y": 892.3842922321207,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 26.000000953674316,
			"height": 0,
			"seed": 25484169,
			"groupIds": [
				"x0LvFsCJmw6ka235WQ6IF"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					26.000000953674316,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 697,
			"versionNonce": 1547074183,
			"isDeleted": false,
			"id": "6PTHUK5Sk5-ZhiZ_0lmIh",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -953.8213139840609,
			"y": 911.3843074909098,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 12.999999046325684,
			"height": 15,
			"seed": 626164329,
			"groupIds": [
				"x0LvFsCJmw6ka235WQ6IF"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					-12.999999046325684,
					15
				]
			]
		},
		{
			"type": "line",
			"version": 697,
			"versionNonce": 74028393,
			"isDeleted": false,
			"id": "Pm32lY0VzlnA1q6FK2pJY",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -953.8213139840609,
			"y": 911.3843074909098,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 13.000001907348633,
			"height": 15,
			"seed": 310922569,
			"groupIds": [
				"x0LvFsCJmw6ka235WQ6IF"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					13.000001907348633,
					15
				]
			]
		},
		{
			"type": "text",
			"version": 697,
			"versionNonce": 11768231,
			"isDeleted": false,
			"id": "e0BfIBhn",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -969.7764130898959,
			"y": 927.3793941608317,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 29.55859375,
			"height": 21,
			"seed": 813808681,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "gt_8of0EwYJZfwTcFxesQ",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "User",
			"rawText": "User",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "User",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 697,
			"versionNonce": 1128595529,
			"isDeleted": false,
			"id": "EcQJ8E92SrcuwO30j3nSe",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -948.4664117166049,
			"y": 867.1143032184489,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 716833417,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 697,
			"versionNonce": 1069597895,
			"isDeleted": false,
			"id": "i0dTfjviwBYXpYH8t8TH1",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -948.4664117166049,
			"y": 867.1143032184489,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 536083369,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 259,
			"versionNonce": 758124329,
			"isDeleted": false,
			"id": "mJ4sRgOMXDhSR282G8oQ8",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3902.1457693109996,
			"y": -1510.8479245337244,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 734790055,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 1458,
			"versionNonce": 409931593,
			"isDeleted": false,
			"id": "3g4D7a3EZDde3GntGAZT1",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -502.1710343066488,
			"y": 385.84359397305,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 568.6816095578276,
			"height": 274.9393605093889,
			"seed": 1018627047,
			"groupIds": [
				"sEitQcNNNuY5TTO9mLfI-"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834797006,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					153.31011892233116,
					0
				],
				[
					231.46097110385094,
					0
				],
				[
					232.72690393419006,
					0.3392056795614167
				],
				[
					233.32431301521768,
					0.7785512362442211
				],
				[
					234.4055675048036,
					1.9427394093064771
				],
				[
					235.30661291279188,
					3.431561236106265
				],
				[
					251.83901478153155,
					44.3903880255952
				],
				[
					565.8007502288659,
					46.23498309200472
				],
				[
					565.8007502288659,
					46.23498309200472
				],
				[
					566.3614487291219,
					46.642357194526234
				],
				[
					566.8193196820404,
					47.75332569842801
				],
				[
					567.1280133244918,
					49.4011420778624
				],
				[
					567.2411798933468,
					51.4189965255678
				],
				[
					567.2411798933468,
					269.75534707582585
				],
				[
					567.2411798933468,
					269.75534707582585
				],
				[
					567.1280133244918,
					271.7732015235312
				],
				[
					566.8193196820404,
					273.4209862622586
				],
				[
					566.3614487291219,
					274.5319547661604
				],
				[
					565.8007502288659,
					274.9393605093889
				],
				[
					0,
					274.9393605093889
				],
				[
					0,
					274.9393605093889
				],
				[
					-0.5606809168859974,
					274.5319547661604
				],
				[
					-1.018538682277025,
					273.4209862622586
				],
				[
					-1.3272323247285125,
					271.7732015235312
				],
				[
					-1.4404296644808512,
					269.75534707582585
				],
				[
					-1.4404296644808512,
					5.184013433563082
				],
				[
					-1.4404296644808512,
					5.184013433563082
				],
				[
					-1.3272323247285125,
					3.1661589858576864
				],
				[
					-1.018538682277025,
					1.5183742471302806
				],
				[
					-0.5606809168859974,
					0.4073741025215129
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 872,
			"versionNonce": 144065033,
			"isDeleted": false,
			"id": "Y0zuSBnx",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -500.79574480619965,
			"y": 386.83869590176096,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 212.4541015625,
			"height": 21,
			"seed": 119271175,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Auto-Debugger Core Components",
			"rawText": "Auto-Debugger Core Components",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Auto-Debugger Core Components",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 723,
			"versionNonce": 611155719,
			"isDeleted": false,
			"id": "COUrdN3TE4dxr0kqUqutb",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -488.6757420596176,
			"y": 580.4355435164664,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 227.7559051513672,
			"height": 62.59379959106445,
			"seed": 133592615,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "2r4Sg-eH9qsNgFWvE1Cm5",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 721,
			"versionNonce": 1114839273,
			"isDeleted": false,
			"id": "1hrBpyfSI5xOeby1XvlIw",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -280.91983690825043,
			"y": 585.4355435164664,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 372447559,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 721,
			"versionNonce": 613811751,
			"isDeleted": false,
			"id": "Xt7jCKLoSBwlSOPV5tIsq",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -282.91983690825043,
			"y": 587.4355435164664,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1999525991,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 721,
			"versionNonce": 475141065,
			"isDeleted": false,
			"id": "dpICc6uzhOs2u50zDLTAe",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -282.91983690825043,
			"y": 591.4355435164664,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 280277895,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 720,
			"versionNonce": 408987975,
			"isDeleted": false,
			"id": "qjEfp259",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -473.6757420596176,
			"y": 599.4306454451773,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 112.0478515625,
			"height": 21,
			"seed": 1526901415,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Java Instrumentor",
			"rawText": "Java Instrumentor",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Java Instrumentor",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 723,
			"versionNonce": 441365161,
			"isDeleted": false,
			"id": "Aq1KmIiK",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -473.6757420596176,
			"y": 615.7275509627555,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 158.689453125,
			"height": 21,
			"seed": 314883527,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "NLUgFUUUbuftk27gi1SkW",
					"type": "arrow"
				},
				{
					"id": "2r4Sg-eH9qsNgFWvE1Cm5",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "(from `instrumentor-java`)",
			"rawText": "(from `instrumentor-java`)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "(from `instrumentor-java`)",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 785,
			"versionNonce": 2099898471,
			"isDeleted": false,
			"id": "g_H-_oc9-LLuP_XbHNS2i",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -143.92389903469632,
			"y": 579.4167113256137,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 191.19729614257812,
			"height": 62.59379959106445,
			"seed": 722955495,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "k8AjP8s7jNR7RHnr5F9cL",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 783,
			"versionNonce": 823424393,
			"isDeleted": false,
			"id": "iH1hXU_2V3Bx12XY-WVO5",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 27.27342762545993,
			"y": 584.4167113256137,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 2139568135,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 783,
			"versionNonce": 1891517319,
			"isDeleted": false,
			"id": "jwflH22z4MTrSlfTAlb7g",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 25.27342762545993,
			"y": 586.4167113256137,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1175494439,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 783,
			"versionNonce": 870243433,
			"isDeleted": false,
			"id": "2p0E-pBIuJGHJP8Xqk5l_",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 25.27342762545993,
			"y": 590.4167113256137,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1092624967,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 787,
			"versionNonce": 593115815,
			"isDeleted": false,
			"id": "VIe6d3H2",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -128.92389903469632,
			"y": 598.4118132543247,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 87.9306640625,
			"height": 21,
			"seed": 831668583,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Java Analyzer",
			"rawText": "Java Analyzer",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Java Analyzer",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 788,
			"versionNonce": 41510729,
			"isDeleted": false,
			"id": "RJJ6Y8Fe",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -128.92389903469632,
			"y": 614.7087187719028,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 133.7998046875,
			"height": 21,
			"seed": 145980551,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "L3xGcwVQSlr5vAEQpUQB2",
					"type": "arrow"
				},
				{
					"id": "IT9cTWL12SVwK35dw1YUZ",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "(from `analyzer-java`)",
			"rawText": "(from `analyzer-java`)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "(from `analyzer-java`)",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 1007,
			"versionNonce": 162155975,
			"isDeleted": false,
			"id": "ebcDVqMeVgyBx_9Ac0WiX",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -487.2539127808492,
			"y": 439.20169398762175,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 237.71580505371094,
			"height": 62.59379959106445,
			"seed": 1514165159,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "k8AjP8s7jNR7RHnr5F9cL",
					"type": "arrow"
				},
				{
					"id": "lCnkaDDsXaJ-Q3lLvPjIN",
					"type": "arrow"
				},
				{
					"id": "AyugQ5Lk28SwkEuNXK3aE",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1003,
			"versionNonce": 2000655913,
			"isDeleted": false,
			"id": "tNp5kgYFcYAwAsn0Di9OX",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -269.5380924683492,
			"y": 444.20169398762175,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 241518279,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1003,
			"versionNonce": 1321925863,
			"isDeleted": false,
			"id": "E6wtwCjC2hNnA-cDLhNKj",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -271.5380924683492,
			"y": 446.20169398762175,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1405015527,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1003,
			"versionNonce": 354216201,
			"isDeleted": false,
			"id": "mtmtXJ-qwSnUP9DsQx0Oz",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -271.5380924683492,
			"y": 450.20169398762175,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1364820231,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1002,
			"versionNonce": 105433095,
			"isDeleted": false,
			"id": "rMOQXxM3",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -472.2539127808492,
			"y": 458.1967959163327,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 126.0615234375,
			"height": 21,
			"seed": 414304295,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Java Test Generator",
			"rawText": "Java Test Generator",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Java Test Generator",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 1010,
			"versionNonce": 1972166633,
			"isDeleted": false,
			"id": "MNXjsA0l",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -472.2539127808492,
			"y": 474.4937014339108,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 168.041015625,
			"height": 21,
			"seed": 1471425351,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "k8AjP8s7jNR7RHnr5F9cL",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "(from `test-generator-java`)",
			"rawText": "(from `test-generator-java`)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "(from `test-generator-java`)",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 723,
			"versionNonce": 505425703,
			"isDeleted": false,
			"id": "REKuyl81n7UWqqz1QZumP",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -353.9257420596176,
			"y": 579.9655422957633,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1253911143,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 1005,
			"versionNonce": 1989868233,
			"isDeleted": false,
			"id": "4RodSqSFjI-f0SQos0RT0",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -487.41388592538044,
			"y": 470.50169703937956,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1781321671,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 687,
			"versionNonce": 280867399,
			"isDeleted": false,
			"id": "dVKpYW4N-LQeNBhponoyg",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -512.1207534430392,
			"y": -46.76270177931036,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1569638727,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747767,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 1394,
			"versionNonce": 685947305,
			"isDeleted": false,
			"id": "Xov1aFklQprShFi4Pr4ix",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -498.4528665340205,
			"y": -105.24466485178021,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 448,
			"height": 113.29000091552734,
			"seed": 932448359,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "jmbEZGJVbPLZ2KtJrb5_I",
					"type": "arrow"
				},
				{
					"id": "gBr8RPfhg9T8G2uo_6b6Y",
					"type": "arrow"
				}
			],
			"updated": 1750834747767,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1392,
			"versionNonce": 1459474791,
			"isDeleted": false,
			"id": "yssLmoop",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -325.0011453426142,
			"y": -104.24957818185834,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 83.26171875,
			"height": 21,
			"seed": 417620871,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "analyzer-java",
			"rawText": "analyzer-java",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "analyzer-java",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 1382,
			"versionNonce": 2106563721,
			"isDeleted": false,
			"id": "jjbNnCk-qOihwLXe0mPjl",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -474.572861651208,
			"y": -62.244664851780215,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 102.24120330810547,
			"height": 46.29690170288086,
			"seed": 565992103,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1381,
			"versionNonce": 439159943,
			"isDeleted": false,
			"id": "YRCewppbUoZCu0b-itbBf",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -392.331650713708,
			"y": -57.244664851780215,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 919104967,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1381,
			"versionNonce": 878519145,
			"isDeleted": false,
			"id": "iPo6xw4lGjZAbLyDyOJP3",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -394.331650713708,
			"y": -55.244664851780215,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1931812071,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1381,
			"versionNonce": 638051239,
			"isDeleted": false,
			"id": "cemf-G9t6XGDKxzsa-lbP",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -394.331650713708,
			"y": -51.244664851780215,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1516169223,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1380,
			"versionNonce": 160878153,
			"isDeleted": false,
			"id": "SDzMdPm5",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -459.572861651208,
			"y": -43.249547664280215,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 55.2412109375,
			"height": 21,
			"seed": 1249005351,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Collector",
			"rawText": "Collector",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Collector",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 1381,
			"versionNonce": 448452295,
			"isDeleted": false,
			"id": "qJlF9GP11enNTwiPJLot5",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -337.572861651208,
			"y": -59.23468560373334,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 278.5244140625,
			"height": 40.265594482421875,
			"seed": 768332359,
			"groupIds": [
				"c2d1vsPz8707nB-HAyels"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					16.130035400390625
				],
				[
					-34.28997802734375,
					20.130035400390625
				],
				[
					0,
					24.130035400390625
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					0,
					40.265594482421875
				],
				[
					244.23443603515625,
					40.265594482421875
				],
				[
					244.23443603515625,
					40.265594482421875
				],
				[
					244.23443603515625,
					40.265594482421875
				],
				[
					244.23443603515625,
					10.000030517578125
				],
				[
					234.23443603515625,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 1414,
			"versionNonce": 605560105,
			"isDeleted": false,
			"id": "EZOQvDmISkZOS66_7cTL_",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -103.33842561605172,
			"y": -59.23468560373334,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 10,
			"height": 10.000030517578125,
			"seed": 1833431399,
			"groupIds": [
				"oA-82lTMEf304Av7kB-B2"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					10.000030517578125
				],
				[
					10,
					10.000030517578125
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 1379,
			"versionNonce": 1591895527,
			"isDeleted": false,
			"id": "2Iseuy11",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -331.572861651208,
			"y": -55.167760554905215,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 193.654296875,
			"height": 19.5,
			"seed": 573779079,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "Concrete analyzer that processes",
			"rawText": "Concrete analyzer that processes",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Concrete analyzer that processes",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 1379,
			"versionNonce": 1129091081,
			"isDeleted": false,
			"id": "dnB5CUet",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -331.572861651208,
			"y": -40.034948054905215,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 193.64794921875,
			"height": 19.5,
			"seed": 139068327,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 13,
			"fontFamily": 2,
			"text": "instrumentation output from DiSL.",
			"rawText": "instrumentation output from DiSL.",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "instrumentation output from DiSL.",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 1381,
			"versionNonce": 1368068359,
			"isDeleted": false,
			"id": "x3Fv07GBf-erMc7xbSdMU",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -67.52592561605172,
			"y": -105.62347954904584,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 800650951,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 1381,
			"versionNonce": 758097641,
			"isDeleted": false,
			"id": "-NhvSkrzGKPQhPDS05_NY",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -65.13554475667672,
			"y": -105.27567071115521,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1864140263,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "arrow",
			"version": 110,
			"versionNonce": 1009866791,
			"isDeleted": false,
			"id": "gBr8RPfhg9T8G2uo_6b6Y",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -278.78141826339845,
			"y": 11.800773532411881,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0.48579285967650776,
			"height": 66.84483502669838,
			"seed": 1654243273,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "Xov1aFklQprShFi4Pr4ix",
				"focus": 0.021244484352253047,
				"gap": 3.755437468664752
			},
			"endBinding": {
				"elementId": "1jBFuHJn",
				"focus": 0.04732340029992546,
				"gap": 4.102864019447793
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					0.48579285967650776,
					66.84483502669838
				]
			]
		},
		{
			"type": "line",
			"version": 812,
			"versionNonce": 1548542409,
			"isDeleted": false,
			"id": "5oE_R4HQUg8tGfkJ0vxAX",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -250.6944428442074,
			"y": 788.029781082046,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 425.0000305175781,
			"height": 131.58999633789062,
			"seed": 1916576297,
			"groupIds": [
				"2I_QwptoZoSjeqPegZVAx"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					375.8896179199219,
					0
				],
				[
					375.8896179199219,
					0
				],
				[
					376.7347106933594,
					0.375457763671875
				],
				[
					377.4527282714844,
					0.9368896484375
				],
				[
					378.0141296386719,
					1.654876708984375
				],
				[
					378.3896179199219,
					2.5
				],
				[
					385.3896179199219,
					22.296905517578125
				],
				[
					420.0000305175781,
					22.296905517578125
				],
				[
					420.0000305175781,
					22.296905517578125
				],
				[
					420.9731140136719,
					22.493377685546875
				],
				[
					421.7677917480469,
					23.029144287109375
				],
				[
					422.3035583496094,
					23.82379150390625
				],
				[
					422.5000305175781,
					24.796905517578125
				],
				[
					422.5000305175781,
					129.08999633789062
				],
				[
					422.5000305175781,
					129.08999633789062
				],
				[
					422.3035583496094,
					130.0631103515625
				],
				[
					421.7677917480469,
					130.85775756835938
				],
				[
					420.9731140136719,
					131.3935546875
				],
				[
					420.0000305175781,
					131.58999633789062
				],
				[
					0,
					131.58999633789062
				],
				[
					0,
					131.58999633789062
				],
				[
					-0.973114013671875,
					131.3935546875
				],
				[
					-1.76776123046875,
					130.85775756835938
				],
				[
					-2.30352783203125,
					130.0631103515625
				],
				[
					-2.5,
					129.08999633789062
				],
				[
					-2.5,
					2.5
				],
				[
					-2.5,
					2.5
				],
				[
					-2.30352783203125,
					1.526885986328125
				],
				[
					-1.76776123046875,
					0.73223876953125
				],
				[
					-0.973114013671875,
					0.19647216796875
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 808,
			"versionNonce": 79805255,
			"isDeleted": false,
			"id": "QKCLfsXG",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -249.1944428442074,
			"y": 789.024898269546,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 291.771484375,
			"height": 21,
			"seed": 1150842121,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Instrumentation & Communication Frameworks",
			"rawText": "Instrumentation & Communication Frameworks",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Instrumentation & Communication Frameworks",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 811,
			"versionNonce": 1841283241,
			"isDeleted": false,
			"id": "pWu5n6OG0AwqaMxYiPjDs",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -237.36442575436365,
			"y": 849.6697957304835,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 154.32420349121094,
			"height": 46.29690170288086,
			"seed": 1169615849,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "L3xGcwVQSlr5vAEQpUQB2",
					"type": "arrow"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 809,
			"versionNonce": 530102887,
			"isDeleted": false,
			"id": "8rhCj0B4oMbMXMr3Fj2kc",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -103.04023752194178,
			"y": 854.6697957304835,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 1302562505,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 809,
			"versionNonce": 1328792457,
			"isDeleted": false,
			"id": "tc53Ni1jX6XlD6_0FwlYy",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -105.04023752194178,
			"y": 856.6697957304835,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 653903273,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 809,
			"versionNonce": 1099093383,
			"isDeleted": false,
			"id": "_7XT2RfhYc6Iz4T5RBzhA",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -105.04023752194178,
			"y": 860.6697957304835,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1021651081,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 810,
			"versionNonce": 753166953,
			"isDeleted": false,
			"id": "oyN37qQE",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -222.36442575436365,
			"y": 868.6648824004054,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 104.2412109375,
			"height": 21,
			"seed": 153169769,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "NLUgFUUUbuftk27gi1SkW",
					"type": "arrow"
				},
				{
					"id": "e9vHrN6FUWDY5oC3YhWmH",
					"type": "arrow"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "DiSL Framework",
			"rawText": "DiSL Framework",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "DiSL Framework",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 812,
			"versionNonce": 1371311271,
			"isDeleted": false,
			"id": "I4AfMP4W1Jvocy1ly2-hr",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -47.794448947723026,
			"y": 852.029781082046,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 203.1962890625,
			"height": 61.59368896484375,
			"seed": 1106860617,
			"groupIds": [
				"5vUISAHJZLUm2nn0nzJTx"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0.29144287109375,
					-0.90850830078125
				],
				[
					1.1409912109375,
					-1.76025390625
				],
				[
					2.51141357421875,
					-2.55706787109375
				],
				[
					4.36553955078125,
					-3.30078125
				],
				[
					6.6661376953125,
					-3.99322509765625
				],
				[
					9.37603759765625,
					-4.63623046875
				],
				[
					12.45794677734375,
					-5.23162841796875
				],
				[
					15.87469482421875,
					-5.78125
				],
				[
					19.58917236328125,
					-6.28692626953125
				],
				[
					23.56402587890625,
					-6.75048828125
				],
				[
					27.76214599609375,
					-7.17376708984375
				],
				[
					32.14630126953125,
					-7.55859375
				],
				[
					41.3238525390625,
					-8.22021484375
				],
				[
					50.799072265625,
					-8.75
				],
				[
					60.2742919921875,
					-9.16259765625
				],
				[
					69.45184326171875,
					-9.47265625
				],
				[
					73.83599853515625,
					-9.59381103515625
				],
				[
					78.03411865234375,
					-9.69482421875
				],
				[
					82.00897216796875,
					-9.77752685546875
				],
				[
					85.723388671875,
					-9.84375
				],
				[
					89.14019775390625,
					-9.89532470703125
				],
				[
					92.22210693359375,
					-9.93408203125
				],
				[
					94.9320068359375,
					-9.96185302734375
				],
				[
					97.2325439453125,
					-9.98046875
				],
				[
					99.086669921875,
					-9.99176025390625
				],
				[
					100.45709228515625,
					-9.99755859375
				],
				[
					101.306640625,
					-9.99969482421875
				],
				[
					101.59814453125,
					-10
				],
				[
					101.88958740234375,
					-9.99969482421875
				],
				[
					102.7391357421875,
					-9.99755859375
				],
				[
					104.10955810546875,
					-9.99176025390625
				],
				[
					105.96368408203125,
					-9.98046875
				],
				[
					108.2642822265625,
					-9.96185302734375
				],
				[
					110.97412109375,
					-9.93408203125
				],
				[
					114.0560302734375,
					-9.89532470703125
				],
				[
					117.47283935546875,
					-9.84375
				],
				[
					121.187255859375,
					-9.77752685546875
				],
				[
					125.16217041015625,
					-9.69482421875
				],
				[
					129.36029052734375,
					-9.59381103515625
				],
				[
					133.74444580078125,
					-9.47265625
				],
				[
					142.9219970703125,
					-9.16259765625
				],
				[
					152.397216796875,
					-8.75
				],
				[
					161.8724365234375,
					-8.22021484375
				],
				[
					171.04998779296875,
					-7.55859375
				],
				[
					175.43414306640625,
					-7.17376708984375
				],
				[
					179.63226318359375,
					-6.75048828125
				],
				[
					183.607177734375,
					-6.28692626953125
				],
				[
					187.32159423828125,
					-5.78125
				],
				[
					190.7384033203125,
					-5.23162841796875
				],
				[
					193.8203125,
					-4.63623046875
				],
				[
					196.5301513671875,
					-3.99322509765625
				],
				[
					198.83074951171875,
					-3.30078125
				],
				[
					200.68487548828125,
					-2.55706787109375
				],
				[
					202.0552978515625,
					-1.76025390625
				],
				[
					202.90484619140625,
					-0.90850830078125
				],
				[
					203.1962890625,
					0
				],
				[
					203.1962890625,
					41.59368896484375
				],
				[
					203.1962890625,
					41.59368896484375
				],
				[
					202.90484619140625,
					42.502197265625
				],
				[
					202.0552978515625,
					43.35394287109375
				],
				[
					200.68487548828125,
					44.1507568359375
				],
				[
					198.83074951171875,
					44.89447021484375
				],
				[
					196.5301513671875,
					45.5869140625
				],
				[
					193.8203125,
					46.22991943359375
				],
				[
					190.7384033203125,
					46.8253173828125
				],
				[
					187.32159423828125,
					47.37493896484375
				],
				[
					183.607177734375,
					47.880615234375
				],
				[
					179.63226318359375,
					48.34417724609375
				],
				[
					175.43414306640625,
					48.7674560546875
				],
				[
					171.04998779296875,
					49.15228271484375
				],
				[
					161.8724365234375,
					49.81390380859375
				],
				[
					152.397216796875,
					50.34368896484375
				],
				[
					142.9219970703125,
					50.75628662109375
				],
				[
					133.74444580078125,
					51.06634521484375
				],
				[
					129.36029052734375,
					51.1875
				],
				[
					125.16217041015625,
					51.28851318359375
				],
				[
					121.187255859375,
					51.3712158203125
				],
				[
					117.47283935546875,
					51.43743896484375
				],
				[
					114.0560302734375,
					51.489013671875
				],
				[
					110.97412109375,
					51.52777099609375
				],
				[
					108.2642822265625,
					51.5555419921875
				],
				[
					105.96368408203125,
					51.57415771484375
				],
				[
					104.10955810546875,
					51.58544921875
				],
				[
					102.7391357421875,
					51.59124755859375
				],
				[
					101.88958740234375,
					51.5933837890625
				],
				[
					101.59814453125,
					51.59368896484375
				],
				[
					101.306640625,
					51.5933837890625
				],
				[
					100.45709228515625,
					51.59124755859375
				],
				[
					99.086669921875,
					51.58544921875
				],
				[
					97.2325439453125,
					51.57415771484375
				],
				[
					94.9320068359375,
					51.5555419921875
				],
				[
					92.22210693359375,
					51.52777099609375
				],
				[
					89.14019775390625,
					51.489013671875
				],
				[
					85.723388671875,
					51.43743896484375
				],
				[
					82.00897216796875,
					51.3712158203125
				],
				[
					78.03411865234375,
					51.28851318359375
				],
				[
					73.83599853515625,
					51.1875
				],
				[
					69.45184326171875,
					51.06634521484375
				],
				[
					60.2742919921875,
					50.75628662109375
				],
				[
					50.799072265625,
					50.34368896484375
				],
				[
					41.3238525390625,
					49.81390380859375
				],
				[
					32.14630126953125,
					49.15228271484375
				],
				[
					27.76214599609375,
					48.7674560546875
				],
				[
					23.56402587890625,
					48.34417724609375
				],
				[
					19.58917236328125,
					47.880615234375
				],
				[
					15.87469482421875,
					47.37493896484375
				],
				[
					12.45794677734375,
					46.8253173828125
				],
				[
					9.37603759765625,
					46.22991943359375
				],
				[
					6.6661376953125,
					45.5869140625
				],
				[
					4.36553955078125,
					44.89447021484375
				],
				[
					2.51141357421875,
					44.1507568359375
				],
				[
					1.1409912109375,
					43.35394287109375
				],
				[
					0.29144287109375,
					42.502197265625
				],
				[
					0,
					41.59368896484375
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 811,
			"versionNonce": 530396489,
			"isDeleted": false,
			"id": "4TCrTighVyCtKf289XdQp",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -47.794448947723026,
			"y": 852.029781082046,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 203.1962890625,
			"height": 10,
			"seed": 1212754217,
			"groupIds": [
				"_FufHDW6WQkY7sJmgUVjW"
			],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				],
				[
					0.29144287109375,
					0.90850830078125
				],
				[
					1.1409912109375,
					1.76025390625
				],
				[
					2.51141357421875,
					2.55706787109375
				],
				[
					4.36553955078125,
					3.30078125
				],
				[
					6.6661376953125,
					3.99322509765625
				],
				[
					9.37603759765625,
					4.63623046875
				],
				[
					12.45794677734375,
					5.23162841796875
				],
				[
					15.87469482421875,
					5.78125
				],
				[
					19.58917236328125,
					6.28692626953125
				],
				[
					23.56402587890625,
					6.75048828125
				],
				[
					27.76214599609375,
					7.17376708984375
				],
				[
					32.14630126953125,
					7.55859375
				],
				[
					41.3238525390625,
					8.22021484375
				],
				[
					50.799072265625,
					8.75
				],
				[
					60.2742919921875,
					9.16259765625
				],
				[
					69.45184326171875,
					9.47265625
				],
				[
					73.83599853515625,
					9.59381103515625
				],
				[
					78.03411865234375,
					9.69482421875
				],
				[
					82.00897216796875,
					9.77752685546875
				],
				[
					85.723388671875,
					9.84375
				],
				[
					89.14019775390625,
					9.89532470703125
				],
				[
					92.22210693359375,
					9.93408203125
				],
				[
					94.9320068359375,
					9.96185302734375
				],
				[
					97.2325439453125,
					9.98046875
				],
				[
					99.086669921875,
					9.99176025390625
				],
				[
					100.45709228515625,
					9.99755859375
				],
				[
					101.306640625,
					9.99969482421875
				],
				[
					101.59814453125,
					10
				],
				[
					101.88958740234375,
					9.99969482421875
				],
				[
					102.7391357421875,
					9.99755859375
				],
				[
					104.10955810546875,
					9.99176025390625
				],
				[
					105.96368408203125,
					9.98046875
				],
				[
					108.2642822265625,
					9.96185302734375
				],
				[
					110.97412109375,
					9.93408203125
				],
				[
					114.0560302734375,
					9.89532470703125
				],
				[
					117.47283935546875,
					9.84375
				],
				[
					121.187255859375,
					9.77752685546875
				],
				[
					125.16217041015625,
					9.69482421875
				],
				[
					129.36029052734375,
					9.59381103515625
				],
				[
					133.74444580078125,
					9.47265625
				],
				[
					142.9219970703125,
					9.16259765625
				],
				[
					152.397216796875,
					8.75
				],
				[
					161.8724365234375,
					8.22021484375
				],
				[
					171.04998779296875,
					7.55859375
				],
				[
					175.43414306640625,
					7.17376708984375
				],
				[
					179.63226318359375,
					6.75048828125
				],
				[
					183.607177734375,
					6.28692626953125
				],
				[
					187.32159423828125,
					5.78125
				],
				[
					190.7384033203125,
					5.23162841796875
				],
				[
					193.8203125,
					4.63623046875
				],
				[
					196.5301513671875,
					3.99322509765625
				],
				[
					198.83074951171875,
					3.30078125
				],
				[
					200.68487548828125,
					2.55706787109375
				],
				[
					202.0552978515625,
					1.76025390625
				],
				[
					202.90484619140625,
					0.90850830078125
				],
				[
					203.1962890625,
					0
				]
			]
		},
		{
			"type": "text",
			"version": 841,
			"versionNonce": 468869063,
			"isDeleted": false,
			"id": "mfdOMRdX",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -17.141404795581707,
			"y": 859.1240285117914,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 71.5927734375,
			"height": 21,
			"seed": 1434197001,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "ShadowVM",
			"rawText": "ShadowVM",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "ShadowVM",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 863,
			"versionNonce": 448373801,
			"isDeleted": false,
			"id": "heoUEDFU",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -22.058796260377108,
			"y": 875.4209035117912,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 162.626953125,
			"height": 21,
			"seed": 1770894057,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "rQ4yaZriSwtV2U2trdnAT",
					"type": "arrow"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "(Communication Channel)",
			"rawText": "(Communication Channel)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "(Communication Channel)",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 811,
			"versionNonce": 1111543527,
			"isDeleted": false,
			"id": "lDtY_--KNtek2sGzIdstf",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -217.99443063717615,
			"y": 849.1997945097804,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 2030661065,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 811,
			"versionNonce": 438646537,
			"isDeleted": false,
			"id": "8XJazwi47klfmsSDFhwPB",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 155.58555593508947,
			"y": 856.1097981718898,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1527434409,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 811,
			"versionNonce": 1865701895,
			"isDeleted": false,
			"id": "TukJxVGhZXsPSDSwYIBTz",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 25.065536403839474,
			"y": 841.779781082046,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 2000506761,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "rectangle",
			"version": 683,
			"versionNonce": 274427369,
			"isDeleted": false,
			"id": "8m6pVprEGLdd1sKZLFapI",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -733.0318580546643,
			"y": 856.8059407000582,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 205.92190551757812,
			"height": 62.59379959106445,
			"seed": 599538311,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "qdiiIkeKH917XjqwzjG8m",
					"type": "arrow"
				},
				{
					"id": "2r4Sg-eH9qsNgFWvE1Cm5",
					"type": "arrow"
				},
				{
					"id": "lCnkaDDsXaJ-Q3lLvPjIN",
					"type": "arrow"
				},
				{
					"id": "b-hdk3tSRfh4yX3dLj3-6",
					"type": "arrow"
				},
				{
					"id": "GMkupK2Y1GU7zoy_2AIzV",
					"type": "arrow"
				},
				{
					"id": "AyugQ5Lk28SwkEuNXK3aE",
					"type": "arrow"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 676,
			"versionNonce": 1204388135,
			"isDeleted": false,
			"id": "M9_Rw9tg7BLnDpUMTQOEN",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -547.1099830546643,
			"y": 861.8059407000582,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 1367429543,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 676,
			"versionNonce": 342345929,
			"isDeleted": false,
			"id": "pA4WYXcMRRLaUt00hehgB",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -549.1099830546643,
			"y": 863.8059407000582,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 819365063,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 676,
			"versionNonce": 493667399,
			"isDeleted": false,
			"id": "wQU8ByEjuH5wG9mUL0vQG",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -549.1099830546643,
			"y": 867.8059407000582,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 1364743143,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 675,
			"versionNonce": 1340458921,
			"isDeleted": false,
			"id": "ExXRdwXk",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -718.0318580546643,
			"y": 875.8010426287691,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 135.3857421875,
			"height": 21,
			"seed": 454246151,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Runner / Orchestrator",
			"rawText": "Runner / Orchestrator",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Runner / Orchestrator",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 679,
			"versionNonce": 1610261351,
			"isDeleted": false,
			"id": "VbiL0CIV",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -718.0318580546643,
			"y": 892.0979481463472,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 140.8134765625,
			"height": 21,
			"seed": 1082377767,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "gt_8of0EwYJZfwTcFxesQ",
					"type": "arrow"
				},
				{
					"id": "AyugQ5Lk28SwkEuNXK3aE",
					"type": "arrow"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "(from `runner` module)",
			"rawText": "(from `runner` module)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "(from `runner` module)",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 1156,
			"versionNonce": 755034761,
			"isDeleted": false,
			"id": "MKsa4RUAW9gCwXxiv3y9o",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -135.69163317340815,
			"y": 1061.7109154409295,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 168.5498046875,
			"height": 62.59379959106445,
			"seed": 1888905543,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "e9vHrN6FUWDY5oC3YhWmH",
					"type": "arrow"
				},
				{
					"id": "rQ4yaZriSwtV2U2trdnAT",
					"type": "arrow"
				},
				{
					"id": "GMkupK2Y1GU7zoy_2AIzV",
					"type": "arrow"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1152,
			"versionNonce": 1337958023,
			"isDeleted": false,
			"id": "QObSOekBqe6c3uKzsdCsf",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 12.85817151409185,
			"y": 1066.7109154409295,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 15,
			"height": 10,
			"seed": 1218947175,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1152,
			"versionNonce": 1148498281,
			"isDeleted": false,
			"id": "dvbL65MLnmwYuTtd0F4df",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 10.85817151409185,
			"y": 1068.7109154409295,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 109314951,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 1152,
			"versionNonce": 1870149031,
			"isDeleted": false,
			"id": "SSB22vintxOmxkkwP5v3K",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 10.85817151409185,
			"y": 1072.7109154409295,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4,
			"height": 2,
			"seed": 2053634727,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1154,
			"versionNonce": 193764425,
			"isDeleted": false,
			"id": "GvMDbHyU",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -120.69163317340815,
			"y": 1080.7060173696404,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 111.2890625,
			"height": 21,
			"seed": 956304839,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "Target Application",
			"rawText": "Target Application",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Target Application",
			"lineHeight": 1.5
		},
		{
			"type": "text",
			"version": 1152,
			"versionNonce": 911272135,
			"isDeleted": false,
			"id": "ypfzE3fN",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -120.69163317340815,
			"y": 1097.0029228872186,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 80.458984375,
			"height": 21,
			"seed": 224967911,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "GMkupK2Y1GU7zoy_2AIzV",
					"type": "arrow"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 14,
			"fontFamily": 2,
			"text": "(User's APK)",
			"rawText": "(User's APK)",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "(User's APK)",
			"lineHeight": 1.5
		},
		{
			"type": "line",
			"version": 678,
			"versionNonce": 1109666601,
			"isDeleted": false,
			"id": "77vCYbAROs7vAjRqxMOg0",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -688.6018043437268,
			"y": 856.3259449725191,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 724469767,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 678,
			"versionNonce": 31864807,
			"isDeleted": false,
			"id": "Q-4-xwctwz30WCS6qoTH8",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -688.6018043437268,
			"y": 856.3259449725191,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 541978407,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 744,
			"versionNonce": 547806729,
			"isDeleted": false,
			"id": "vzqeU9OtnuugADBZN-wLJ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -374.69439597243456,
			"y": 373.80189269494736,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 120812103,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 678,
			"versionNonce": 1212457735,
			"isDeleted": false,
			"id": "PII8SowfG7MI0jIWtiGmJ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -678.6518531718518,
			"y": 856.4559345965425,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 2025558375,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "line",
			"version": 1154,
			"versionNonce": 1175467241,
			"isDeleted": false,
			"id": "vGxwht1rCpmPcLIgh8yTb",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 33.32826428752935,
			"y": 1093.0109184926873,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 0,
			"height": 0,
			"seed": 1129685927,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					0,
					0
				]
			]
		},
		{
			"type": "arrow",
			"version": 1568,
			"versionNonce": 166242855,
			"isDeleted": false,
			"id": "qdiiIkeKH917XjqwzjG8m",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -946.7520325356328,
			"y": 865.3886314762034,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 223.9488095152035,
			"height": 85.34583831455359,
			"seed": 1654107175,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "ykwuFh5b"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "e0lxewGjD18Ayu__mobQK",
				"focus": -0.6309141691097307,
				"gap": 5.072080769958198
			},
			"endBinding": {
				"elementId": "8m6pVprEGLdd1sKZLFapI",
				"focus": -0.3242866687213977,
				"gap": 2.578158776994087
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					123.9494722256469,
					-85.34583831455359
				],
				[
					223.9488095152035,
					-11.160849553139315
				]
			]
		},
		{
			"type": "text",
			"version": 64,
			"versionNonce": 1709944777,
			"isDeleted": false,
			"id": "ykwuFh5b",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2369.0522802952196,
			"y": 578.3783276428378,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 160.078125,
			"height": 18.4,
			"seed": 894657257,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "1. Submits Run Config",
			"rawText": "1. Submits Run Config",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "qdiiIkeKH917XjqwzjG8m",
			"originalText": "1. Submits Run Config",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1894,
			"versionNonce": 62836039,
			"isDeleted": false,
			"id": "2r4Sg-eH9qsNgFWvE1Cm5",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -586.1350320903034,
			"y": 853.7103220174439,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 195.49389054827975,
			"height": 207.2171243468266,
			"seed": 777026601,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "MZbH9J7Y"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "8m6pVprEGLdd1sKZLFapI",
				"focus": 0.08671853120557141,
				"gap": 3.0956186826142584
			},
			"endBinding": {
				"elementId": "Aq1KmIiK",
				"focus": -0.2555610304493975,
				"gap": 9.765646707861833
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					195.49389054827975,
					-207.2171243468266
				]
			]
		},
		{
			"type": "text",
			"version": 53,
			"versionNonce": 911527593,
			"isDeleted": false,
			"id": "MZbH9J7Y",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2653.07780000847,
			"y": 658.2507453754157,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 79.1484375,
			"height": 36.8,
			"seed": 110642023,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "2. Invokes\nwith Config",
			"rawText": "2. Invokes\nwith Config",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "2r4Sg-eH9qsNgFWvE1Cm5",
			"originalText": "2. Invokes\nwith Config",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1912,
			"versionNonce": 555977831,
			"isDeleted": false,
			"id": "NLUgFUUUbuftk27gi1SkW",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -376.50342793719517,
			"y": 644.7055765642934,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 138.56117795597902,
			"height": 230.89021110080319,
			"seed": 1710262343,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "9iPlzMCs"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "Aq1KmIiK",
				"focus": -0.21142511446229145,
				"gap": 7.978025601537979
			},
			"endBinding": {
				"elementId": "oyN37qQE",
				"focus": -0.6333370860851621,
				"gap": 15.5778242268525
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					9.808541196613078,
					192.96006683069595
				],
				[
					138.56117795597902,
					230.89021110080319
				]
			]
		},
		{
			"type": "text",
			"version": 73,
			"versionNonce": 502731145,
			"isDeleted": false,
			"id": "9iPlzMCs",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2858.906080511775,
			"y": 847.1145658840082,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 207.2265625,
			"height": 18.4,
			"seed": 1963069255,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "3. Generates Instrumentation",
			"rawText": "3. Generates Instrumentation",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "NLUgFUUUbuftk27gi1SkW",
			"originalText": "3. Generates Instrumentation",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1970,
			"versionNonce": 769877895,
			"isDeleted": false,
			"id": "L3xGcwVQSlr5vAEQpUQB2",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -140.89762833966697,
			"y": 848.6697957304837,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 82.63897236942246,
			"height": 206.8536466003403,
			"seed": 875555145,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "V7skKOZT"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "pWu5n6OG0AwqaMxYiPjDs",
				"focus": 0.22614120291946832,
				"gap": 1
			},
			"endBinding": {
				"elementId": "RJJ6Y8Fe",
				"focus": -0.2439196610486434,
				"gap": 6.107430358240549
			},
			"lastCommittedPoint": null,
			"startArrowhead": "triangle",
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					7.770773353393992,
					-123.07838220176643
				],
				[
					82.63897236942246,
					-206.8536466003403
				]
			]
		},
		{
			"type": "text",
			"version": 54,
			"versionNonce": 571340905,
			"isDeleted": false,
			"id": "V7skKOZT",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3045.080509498754,
			"y": 781.9398159517548,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 169.8671875,
			"height": 18.4,
			"seed": 1388713863,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "4. Runs Instrumentation",
			"rawText": "4. Runs Instrumentation",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "L3xGcwVQSlr5vAEQpUQB2",
			"originalText": "4. Runs Instrumentation",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1784,
			"versionNonce": 1211817639,
			"isDeleted": false,
			"id": "e9vHrN6FUWDY5oC3YhWmH",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -151.87429263567753,
			"y": 898.3881917985336,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 33.93343995106079,
			"height": 162.13948179182444,
			"seed": 2090935529,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "fqACL8OG"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "oyN37qQE",
				"focus": -0.1409807044921258,
				"gap": 8.723309398128208
			},
			"endBinding": {
				"elementId": "MKsa4RUAW9gCwXxiv3y9o",
				"focus": -0.7616822018952133,
				"gap": 1.1832418505714486
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					29.512517395441137,
					55.43837090690829
				],
				[
					33.93343995106079,
					162.13948179182444
				]
			]
		},
		{
			"type": "text",
			"version": 82,
			"versionNonce": 342164297,
			"isDeleted": false,
			"id": "fqACL8OG",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3093.333120960051,
			"y": 1008.5108772890543,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 190.3203125,
			"height": 36.8,
			"seed": 1676809031,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "5. Instruments Bytecode at\nRuntime",
			"rawText": "5. Instruments Bytecode at Runtime",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "e9vHrN6FUWDY5oC3YhWmH",
			"originalText": "5. Instruments Bytecode at Runtime",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1800,
			"versionNonce": 316043719,
			"isDeleted": false,
			"id": "rQ4yaZriSwtV2U2trdnAT",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -6.524018365670486,
			"y": 1061.4047736082923,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 66.21989388491284,
			"height": 156.547800462547,
			"seed": 888969095,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "SwUE28Vi"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "MKsa4RUAW9gCwXxiv3y9o",
				"focus": 0.11011082508403659,
				"gap": 1
			},
			"endBinding": {
				"elementId": "heoUEDFU",
				"focus": -0.023941427648994033,
				"gap": 8.436069633954162
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					58.22281778442539,
					-57.300809800912475
				],
				[
					66.21989388491284,
					-156.547800462547
				]
			]
		},
		{
			"type": "text",
			"version": 55,
			"versionNonce": 22524457,
			"isDeleted": false,
			"id": "SwUE28Vi",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3198.0330487475303,
			"y": 1033.5972629367727,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 174,
			"height": 18.4,
			"seed": 117929449,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "6. Emits Runtime Values",
			"rawText": "6. Emits Runtime Values",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "rQ4yaZriSwtV2U2trdnAT",
			"originalText": "6. Emits Runtime Values",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1270,
			"versionNonce": 651337959,
			"isDeleted": false,
			"id": "IT9cTWL12SVwK35dw1YUZ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -3.8983838407477265,
			"y": 646.8213320465554,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 62.00130086916698,
			"height": 194.6248106972531,
			"seed": 593794919,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "JJeLkZAa"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "RJJ6Y8Fe",
				"focus": -0.5878214371389423,
				"gap": 11.112613274652631
			},
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					52.959931107287105,
					78.26836496655858
				],
				[
					62.00130086916698,
					194.6248106972531
				]
			]
		},
		{
			"type": "text",
			"version": 54,
			"versionNonce": 1574012169,
			"isDeleted": false,
			"id": "JJeLkZAa",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3239.752090682999,
			"y": 767.5420323689414,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 158.2890625,
			"height": 36.8,
			"seed": 1604945863,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "7. Consumes Runtime\nValues",
			"rawText": "7. Consumes Runtime Values",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "IT9cTWL12SVwK35dw1YUZ",
			"originalText": "7. Consumes Runtime Values",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 2622,
			"versionNonce": 1169943559,
			"isDeleted": false,
			"id": "k8AjP8s7jNR7RHnr5F9cL",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -49.07409261977051,
			"y": 577.0629716316081,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 200.36647505304353,
			"height": 106.1800384344408,
			"seed": 1097534537,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "XaUH5Yed"
				}
			],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "g_H-_oc9-LLuP_XbHNS2i",
				"focus": 0.4042330468321769,
				"gap": 2.3537396940056396
			},
			"endBinding": {
				"elementId": "ebcDVqMeVgyBx_9Ac0WiX",
				"focus": -0.6633405660527776,
				"gap": 1
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					-94.11760534639325,
					-50.17171494547051
				],
				[
					-200.36647505304353,
					-106.1800384344408
				]
			]
		},
		{
			"type": "text",
			"version": 81,
			"versionNonce": 1968045033,
			"isDeleted": false,
			"id": "XaUH5Yed",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 3303.212974955987,
			"y": 648.3550281235154,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 130.75,
			"height": 55.199999999999996,
			"seed": 1136038727,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747768,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "8. Sends collected\nExecution Trace\n",
			"rawText": "8. Sends collected Execution Trace \n",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "k8AjP8s7jNR7RHnr5F9cL",
			"originalText": "8. Sends collected Execution Trace \n",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1310,
			"versionNonce": 1622660679,
			"isDeleted": false,
			"id": "gt_8of0EwYJZfwTcFxesQ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -725.4866765286388,
			"y": 920.9255010361624,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 217.10856497908003,
			"height": 79.74967660695745,
			"seed": 1771788073,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "eFVbfjjm"
				}
			],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "VbiL0CIV",
				"focus": 0.41926889169252857,
				"gap": 7.827552889815138
			},
			"endBinding": {
				"elementId": "e0lxewGjD18Ayu__mobQK",
				"focus": 0.38193301521058337,
				"gap": 7.895297591300608
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					-98.815410175985,
					46.46172432144954
				],
				[
					-217.10856497908003,
					-33.28795228550791
				]
			]
		},
		{
			"type": "text",
			"version": 75,
			"versionNonce": 135058857,
			"isDeleted": false,
			"id": "eFVbfjjm",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2309.2807373619617,
			"y": 741.4112215896401,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 163.65625,
			"height": 36.8,
			"seed": 1618870567,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "10. Displays generated\ntests & status",
			"rawText": "10. Displays generated tests & status",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "gt_8of0EwYJZfwTcFxesQ",
			"originalText": "10. Displays generated tests & status",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1577,
			"versionNonce": 1813214567,
			"isDeleted": false,
			"id": "b-hdk3tSRfh4yX3dLj3-6",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -939.9865791853993,
			"y": 880.8267419053745,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 205.1088905578905,
			"height": 7.44862084876911,
			"seed": 1048497129,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "13wJI0qJ"
				}
			],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "e0lxewGjD18Ayu__mobQK",
				"focus": 0.4921799491276496,
				"gap": 6.53049366156197
			},
			"endBinding": {
				"elementId": "8m6pVprEGLdd1sKZLFapI",
				"focus": -0.11355835260301748,
				"gap": 1.8458305728444202
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					205.1088905578905,
					7.44862084876911
				]
			]
		},
		{
			"type": "text",
			"version": 38,
			"versionNonce": 689322121,
			"isDeleted": false,
			"id": "13wJI0qJ",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2271.0347388163054,
			"y": 644.9420555025252,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 65.8125,
			"height": 36.8,
			"seed": 1607402599,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Executes\ntests",
			"rawText": "Executes\ntests",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "b-hdk3tSRfh4yX3dLj3-6",
			"originalText": "Executes\ntests",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1294,
			"versionNonce": 1156460679,
			"isDeleted": false,
			"id": "GMkupK2Y1GU7zoy_2AIzV",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -554.2391436218604,
			"y": 922.0675027344585,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 416.56556162829304,
			"height": 181.5286489510504,
			"seed": 186683849,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "NfaekTNL"
				}
			],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "8m6pVprEGLdd1sKZLFapI",
				"focus": -0.431515627286079,
				"gap": 2.667762443335846
			},
			"endBinding": {
				"elementId": "MKsa4RUAW9gCwXxiv3y9o",
				"focus": -0.5036475141050102,
				"gap": 1.9819488201592321
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					94.95978721705103,
					143.5464827864149
				],
				[
					416.56556162829304,
					181.5286489510504
				]
			]
		},
		{
			"type": "text",
			"version": 53,
			"versionNonce": 1823489897,
			"isDeleted": false,
			"id": "NfaekTNL",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 2603.53815137284,
			"y": 979.9177312952671,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 212.5703125,
			"height": 18.4,
			"seed": 551393833,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "Runs tests against application",
			"rawText": "Runs tests against application",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "GMkupK2Y1GU7zoy_2AIzV",
			"originalText": "Runs tests against application",
			"lineHeight": 1.15
		},
		{
			"type": "text",
			"version": 184,
			"versionNonce": 1934852039,
			"isDeleted": false,
			"id": "S8i1vDXv",
			"fillStyle": "hachure",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -556.0530896796599,
			"y": 306.5214316223535,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 371.3681640625,
			"height": 33,
			"seed": 2110621959,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834808832,
			"link": null,
			"locked": false,
			"fontSize": 22,
			"fontFamily": 2,
			"text": "Components & Connectors Viewpoint ",
			"rawText": "Components & Connectors Viewpoint ",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Components & Connectors Viewpoint ",
			"lineHeight": 1.5
		},
		{
			"type": "rectangle",
			"version": 445,
			"versionNonce": 77924487,
			"isDeleted": false,
			"id": "SoUpk09V2Hufdodv_-9yq",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -989.0304771792823,
			"y": 363.5723477280904,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1189.5172775418519,
			"height": 795.8494073755561,
			"seed": 1631246439,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834805862,
			"link": null,
			"locked": false
		},
		{
			"id": "AyugQ5Lk28SwkEuNXK3aE",
			"type": "arrow",
			"x": -488.40783485642146,
			"y": 471.67078451076065,
			"width": 137.82416186269438,
			"height": 383.9940510474421,
			"angle": 0,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"seed": 1983578409,
			"version": 928,
			"versionNonce": 1062484265,
			"isDeleted": false,
			"boundElements": [
				{
					"type": "text",
					"id": "qiQS1G8S"
				}
			],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"points": [
				[
					0,
					0
				],
				[
					-122.54650776207654,
					72.58858142237841
				],
				[
					-137.82416186269438,
					383.9940510474421
				]
			],
			"lastCommittedPoint": null,
			"startBinding": {
				"elementId": "ebcDVqMeVgyBx_9Ac0WiX",
				"focus": 0.6874592202044121,
				"gap": 1.1539220755722681
			},
			"endBinding": {
				"elementId": "8m6pVprEGLdd1sKZLFapI",
				"focus": 0.02150781934517454,
				"gap": 1.1411051418554052
			},
			"startArrowhead": null,
			"endArrowhead": "triangle"
		},
		{
			"id": "qiQS1G8S",
			"type": "text",
			"x": 68.82523017258563,
			"y": 1128.9476803980247,
			"width": 153.8671875,
			"height": 36.8,
			"angle": 0,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"seed": 285073575,
			"version": 24,
			"versionNonce": 973474279,
			"isDeleted": false,
			"boundElements": null,
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"text": "9. Returns Generated\nTest Suite",
			"rawText": "9. Returns Generated Test Suite",
			"fontSize": 16,
			"fontFamily": 2,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "AyugQ5Lk28SwkEuNXK3aE",
			"originalText": "9. Returns Generated Test Suite",
			"lineHeight": 1.15
		},
		{
			"type": "arrow",
			"version": 1593,
			"versionNonce": 1267651367,
			"isDeleted": true,
			"id": "lCnkaDDsXaJ-Q3lLvPjIN",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 361.2794351251382,
			"y": 518.6897875603014,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 996.9461734209707,
			"height": 615.5943597867439,
			"seed": 1967215273,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "MNXjsA0l",
				"focus": -0.6196811020418822,
				"gap": 6.7375918173280525
			},
			"endBinding": {
				"elementId": "VbiL0CIV",
				"focus": -0.0632881324383576,
				"gap": 12.210112953782186
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle",
			"points": [
				[
					0,
					0
				],
				[
					-839.6376651369628,
					615.5943597867439
				],
				[
					-996.9461734209707,
					281.8429183200459
				]
			]
		},
		{
			"type": "text",
			"version": 64,
			"versionNonce": 400070345,
			"isDeleted": true,
			"id": "uzBiKCi5",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -480.5808862618246,
			"y": 1125.0841473470452,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 4.4453125,
			"height": 18.4,
			"seed": 620905801,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"fontSize": 16,
			"fontFamily": 2,
			"text": "",
			"rawText": "",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "lCnkaDDsXaJ-Q3lLvPjIN",
			"originalText": "",
			"lineHeight": 1.15
		},
		{
			"type": "rectangle",
			"version": 385,
			"versionNonce": 621053863,
			"isDeleted": true,
			"id": "VWWYOf4wWIeZJtb6AHy0B",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -1013.3209327662364,
			"y": 363.46241416412425,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1620.527154636326,
			"height": 795.8494073755561,
			"seed": 657683977,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750834747769,
			"link": null,
			"locked": false
		},
		{
			"id": "dYEJ00gS",
			"type": "text",
			"x": 180.88773017258563,
			"y": 1127.396460146871,
			"width": 4.4453125,
			"height": 18.4,
			"angle": 0,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"seed": 565479561,
			"version": 23,
			"versionNonce": 1487219721,
			"isDeleted": true,
			"boundElements": null,
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"text": "",
			"rawText": "",
			"fontSize": 16,
			"fontFamily": 2,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "AyugQ5Lk28SwkEuNXK3aE",
			"originalText": "",
			"lineHeight": 1.15
		},
		{
			"id": "44QzM6ne",
			"type": "text",
			"x": 180.88773017258563,
			"y": 1127.396460146871,
			"width": 4.4453125,
			"height": 18.4,
			"angle": 0,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"seed": 163810983,
			"version": 23,
			"versionNonce": 1352283399,
			"isDeleted": true,
			"boundElements": null,
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"text": "",
			"rawText": "",
			"fontSize": 16,
			"fontFamily": 2,
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "AyugQ5Lk28SwkEuNXK3aE",
			"originalText": "",
			"lineHeight": 1.15
		},
		{
			"id": "YRoqR7QvoN3ZrOGm7yt4m",
			"type": "arrow",
			"x": 342.886072291401,
			"y": 520.7620204389127,
			"width": 114.3938903488114,
			"height": 619.3988050161065,
			"angle": 0,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"seed": 829877865,
			"version": 150,
			"versionNonce": 1158988521,
			"isDeleted": true,
			"boundElements": null,
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"points": [
				[
					0,
					0
				],
				[
					-40.33172652999224,
					547.5807486572021
				],
				[
					-114.3938903488114,
					619.3988050161065
				]
			],
			"lastCommittedPoint": null,
			"startBinding": {
				"elementId": "MNXjsA0l",
				"focus": -0.24127195453823674,
				"gap": 8.80982469593937
			},
			"endBinding": null,
			"startArrowhead": null,
			"endArrowhead": null
		},
		{
			"id": "MTzJsmmwj6vO6UctznCxb",
			"type": "line",
			"x": 335.1590079301518,
			"y": 517.2915341667465,
			"width": 113.99878936774871,
			"height": 624.8288075472817,
			"angle": 0,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"fillStyle": "solid",
			"strokeWidth": 1,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"seed": 1959765353,
			"version": 159,
			"versionNonce": 1438383143,
			"isDeleted": true,
			"boundElements": null,
			"updated": 1750834747769,
			"link": null,
			"locked": false,
			"points": [
				[
					0,
					0
				],
				[
					-10.101158551572553,
					360.7556625561673
				],
				[
					-113.99878936774871,
					624.8288075472817
				]
			],
			"lastCommittedPoint": null,
			"startBinding": null,
			"endBinding": null,
			"startArrowhead": null,
			"endArrowhead": null
		}
	],
	"appState": {
		"theme": "light",
		"viewBackgroundColor": "#ffffff",
		"currentItemStrokeColor": "#1e1e1e",
		"currentItemBackgroundColor": "transparent",
		"currentItemFillStyle": "solid",
		"currentItemStrokeWidth": 1,
		"currentItemStrokeStyle": "solid",
		"currentItemRoughness": 0,
		"currentItemOpacity": 100,
		"currentItemFontFamily": 2,
		"currentItemFontSize": 16,
		"currentItemTextAlign": "left",
		"currentItemStartArrowhead": "triangle",
		"currentItemEndArrowhead": "triangle",
		"scrollX": 1658.2470301466433,
		"scrollY": 312.6833400324086,
		"zoom": {
			"value": 0.7194532802093284
		},
		"currentItemRoundness": "round",
		"gridSize": null,
		"gridColor": {
			"Bold": "#C9C9C9FF",
			"Regular": "#EDEDEDFF"
		},
		"currentStrokeOptions": null,
		"previousGridSize": null,
		"frameRendering": {
			"enabled": true,
			"clip": true,
			"name": true,
			"outline": true
		}
	},
	"files": {}
}
```
%%