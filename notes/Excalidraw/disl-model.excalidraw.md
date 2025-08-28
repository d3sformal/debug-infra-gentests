---

excalidraw-plugin: parsed
tags: [excalidraw]

---
==⚠  Switch to EXCALIDRAW VIEW in the MORE OPTIONS menu of this document. ⚠==


# Text Elements
DiSL Instrumentation Generating Workflow ^ktMjMaWn

Compiler ^O0QFPg3V

Packager ^pU9leOse

Object Path ^omf7PqHy

Code Generator ^8cUKJT9z

Model ^bJKzixZO

Model::transform ^30bpV1j8

Source Path ^cXheyDWH

TextWriter ^E94ACQrt

JAR ^5gzNiyV8

Create a DiSL Metamodel and use it to enforce structure of the DiSLClass?
    - The code can be generated using a M2T transformation
    - The M2T interface can be shared in the instrumentation-common module
    - The M2T logic for emitting Java could be injected to the common interface ^Lcd8Sjm8

Create a Test Metamodel and use it for generating tests?
    - Some sort of M2T could be reused from the instrumentation ^bCKopXby

DiSLClass Metamodel ^eGA4HeB4

DiSLClass ^XnAlE70p

Package
--------------
- name ^qLoTDm4L

Import ^qyRNEAm9

Instrumentation Method
----------------
- name ^7h5KDroU

Annotation
----------------
- activationTime ^oBdnkh2N

Marker
----------------
- markerClass ^bHAl32JU

Scope
----------------
- methodIdentifier ^K8J6EW1f

Exportable
---------------
- varIdentifier ^MBdKRmYS

exports ^k2N3E6NL

targetMethod ^InvDj0Jn

marker ^kZV7HJ63

annotation ^DimuyQsQ

* ^lfIjSvUb

logic ^b9rOm2t0

* ^I2NAW9hE

package ^mzgZrip5

imports ^xUlhX6dD

Metaclass
----------------
+ visit() ^PGMcL9QY

Model
----------------
- dislClass
+ visit() ^H7fy2xLa

* ^g7hfsbPh

Argument ^RAUFJIgC

Field ^YXjV3ieV

Variable ^WqyLX9QA

DiSL Class Layout ^JjpNZSnW

package %s;

%s

public class %s {

  @After(marker = %s, scope = "%s")
  public static void %s(DynamicContext di) {
      int a = di.getLocalVariableValue(0, int.class);
      int b = di.getLocalVariableValue(1, int.class);
      System.out.println("disl: a=" + a);
      System.out.println("disl: b=" + b);
  }
} ^N1TyDhs9

package ^pzQFrZ9l

PACKAGE_NAME ^unPkX3IS

Imports ^6sa9bsE8

public class ^FEnNVw67

CLASS_NAME ^CyxhAhUr

static void  ^14T2GbeW

INST_METHOD ^ngq5TkdC

ANNOTATION ^lUbAUeQj

MARKER ^Bb4Ac1DY

SCOPE ^qxrWXPlz

INST_LOGIC ^wK0A1Qxj

ARGUMENTS ^qHGqlOKQ

VARIABLES ^dScp8SY6

FIELDS ^KFg8fMyF

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
			"version": 569,
			"versionNonce": 1073542153,
			"isDeleted": false,
			"id": "QrB3nCiVArohjSd0f1XJy",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -509.9136627629194,
			"y": -630.5497594326403,
			"strokeColor": "#1971c2",
			"backgroundColor": "transparent",
			"width": 494.5949124956406,
			"height": 954.583893865281,
			"seed": 794987625,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 227,
			"versionNonce": 25357575,
			"isDeleted": false,
			"id": "ktMjMaWn",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -496.1430940120279,
			"y": -604.4158609209538,
			"strokeColor": "#1971c2",
			"backgroundColor": "transparent",
			"width": 468.75,
			"height": 24,
			"seed": 839325513,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "DiSL Instrumentation Generating Workflow",
			"rawText": "DiSL Instrumentation Generating Workflow",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "DiSL Instrumentation Generating Workflow",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 842,
			"versionNonce": 2117307113,
			"isDeleted": false,
			"id": "-8qdJNXydQeIpWgJ6cj1G",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -405.161962761433,
			"y": -534.1765642298619,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 261.06423738556737,
			"height": 309.4619243282191,
			"seed": 1234650665,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"id": "5Mt8pdQPqk-b0kYzmr2-6",
					"type": "arrow"
				},
				{
					"id": "Do94x1moIkznu_jRQxol0",
					"type": "arrow"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 881,
			"versionNonce": 1489876007,
			"isDeleted": false,
			"id": "RfC5eXQI6G8AOcX6lLcXI",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -371.2788620625943,
			"y": -93.7014795081227,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 224.10568616156525,
			"height": 132.64767138771913,
			"seed": 553556233,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "O0QFPg3V"
				},
				{
					"id": "oR00WqcyFHNqKULRA3ein",
					"type": "arrow"
				},
				{
					"id": "Do94x1moIkznu_jRQxol0",
					"type": "arrow"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 838,
			"versionNonce": 724846025,
			"isDeleted": false,
			"id": "O0QFPg3V",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": -306.1010189818117,
			"y": -39.37764381426314,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 93.75,
			"height": 24,
			"seed": 837866699,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Compiler",
			"rawText": "Compiler",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "RfC5eXQI6G8AOcX6lLcXI",
			"originalText": "Compiler",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 1078,
			"versionNonce": 91395911,
			"isDeleted": false,
			"id": "6jV65uDaRXHqh7u4F9LF3",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -365.3289994935445,
			"y": 162.06431650571403,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 224.10568616156525,
			"height": 132.64767138771913,
			"seed": 2042138601,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "pU9leOse"
				},
				{
					"id": "oR00WqcyFHNqKULRA3ein",
					"type": "arrow"
				},
				{
					"id": "7vp8C1qyiKu3LUzC1XIWu",
					"type": "arrow"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1050,
			"versionNonce": 840606889,
			"isDeleted": false,
			"id": "pU9leOse",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": -300.15115641276185,
			"y": 216.3881521995736,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 93.75,
			"height": 24,
			"seed": 596925963,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Packager",
			"rawText": "Packager",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "6jV65uDaRXHqh7u4F9LF3",
			"originalText": "Packager",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 2611,
			"versionNonce": 231609959,
			"isDeleted": false,
			"id": "oR00WqcyFHNqKULRA3ein",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -268.19485065385163,
			"y": 50.797346847176414,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1.462826617285316,
			"height": 107.30641823795804,
			"seed": 1506847433,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "omf7PqHy"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "RfC5eXQI6G8AOcX6lLcXI",
				"gap": 11.851154967579987,
				"focus": 0.08883498178739785
			},
			"endBinding": {
				"elementId": "6jV65uDaRXHqh7u4F9LF3",
				"gap": 3.960551420579577,
				"focus": -0.11064153401646845
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
					1.462826617285316,
					107.30641823795804
				]
			]
		},
		{
			"type": "text",
			"version": 43,
			"versionNonce": 244459401,
			"isDeleted": false,
			"id": "omf7PqHy",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": -331.91656234520894,
			"y": 92.45055596615543,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 128.90625,
			"height": 24,
			"seed": 1649841995,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Object Path",
			"rawText": "Object Path",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "oR00WqcyFHNqKULRA3ein",
			"originalText": "Object Path",
			"lineHeight": 1.2
		},
		{
			"type": "text",
			"version": 275,
			"versionNonce": 2024583559,
			"isDeleted": false,
			"id": "8cUKJT9z",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -391.7250330109444,
			"y": -509.9103105421859,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 164.0625,
			"height": 24,
			"seed": 2044881321,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Code Generator",
			"rawText": "Code Generator",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Code Generator",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 1222,
			"versionNonce": 987123305,
			"isDeleted": false,
			"id": "5Mt8pdQPqk-b0kYzmr2-6",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -666.5288293748649,
			"y": -430.58912469757297,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 245.06280671474073,
			"height": 0.695150676621779,
			"seed": 1931037833,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "bJKzixZO"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"startBinding": null,
			"endBinding": {
				"elementId": "-8qdJNXydQeIpWgJ6cj1G",
				"focus": 0.336910231294277,
				"gap": 16.304059898691065
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
					245.06280671474073,
					-0.695150676621779
				]
			]
		},
		{
			"type": "text",
			"version": 33,
			"versionNonce": 1061873831,
			"isDeleted": false,
			"id": "bJKzixZO",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": -573.2943010174945,
			"y": -442.93670003588386,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 58.59375,
			"height": 24,
			"seed": 1841379115,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Model",
			"rawText": "Model",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "5Mt8pdQPqk-b0kYzmr2-6",
			"originalText": "Model",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 479,
			"versionNonce": 1571985737,
			"isDeleted": false,
			"id": "h3Ohd508knYORidVTIcLH",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -376.18215767308874,
			"y": -469.9405315726831,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 210.2905233630223,
			"height": 90.31147319314641,
			"seed": 758174569,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "30bpV1j8"
				},
				{
					"id": "7RIwvgRP8QZ2FM5HOnaRv",
					"type": "arrow"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 449,
			"versionNonce": 2136292295,
			"isDeleted": false,
			"id": "30bpV1j8",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": -364.7868959915776,
			"y": -436.7847949761099,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 187.5,
			"height": 24,
			"seed": 1438099563,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Model::transform",
			"rawText": "Model::transform",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "h3Ohd508knYORidVTIcLH",
			"originalText": "Model::transform",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 638,
			"versionNonce": 948078633,
			"isDeleted": false,
			"id": "Do94x1moIkznu_jRQxol0",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -268.9041832466783,
			"y": -217.6335418634692,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 2.6909747972311493,
			"height": 114.29334889187339,
			"seed": 208961097,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "cXheyDWH"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "-8qdJNXydQeIpWgJ6cj1G",
				"gap": 7.08109803817365,
				"focus": -0.014278973624994227
			},
			"endBinding": {
				"elementId": "RfC5eXQI6G8AOcX6lLcXI",
				"gap": 9.638713463473096,
				"focus": -0.04575733747176878
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
					2.6909747972311493,
					114.29334889187339
				]
			]
		},
		{
			"type": "text",
			"version": 53,
			"versionNonce": 1827154663,
			"isDeleted": false,
			"id": "cXheyDWH",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": -332.0118208480627,
			"y": -172.4868674175325,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 128.90625,
			"height": 24,
			"seed": 1899153835,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Source Path",
			"rawText": "Source Path",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "Do94x1moIkznu_jRQxol0",
			"originalText": "Source Path",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 636,
			"versionNonce": 1429314313,
			"isDeleted": false,
			"id": "j0Y-vX54gnnJeI0aIkLaO",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -377.1770418121259,
			"y": -333.70274817502514,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 210.2905233630223,
			"height": 90.31147319314641,
			"seed": 1606405417,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"id": "7RIwvgRP8QZ2FM5HOnaRv",
					"type": "arrow"
				},
				{
					"type": "text",
					"id": "E94ACQrt"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 71,
			"versionNonce": 76559879,
			"isDeleted": false,
			"id": "E94ACQrt",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": -330.6255301306147,
			"y": -300.54701157845193,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 117.1875,
			"height": 24,
			"seed": 1254529771,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "TextWriter",
			"rawText": "TextWriter",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "j0Y-vX54gnnJeI0aIkLaO",
			"originalText": "TextWriter",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 426,
			"versionNonce": 1950044649,
			"isDeleted": false,
			"id": "7RIwvgRP8QZ2FM5HOnaRv",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -273.2386053059813,
			"y": -376.7315383130549,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1.183029599680708,
			"height": 41.95271266376767,
			"seed": 243154953,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "h3Ohd508knYORidVTIcLH",
				"gap": 2.897520066481775,
				"focus": 0.03342243207709898
			},
			"endBinding": {
				"elementId": "j0Y-vX54gnnJeI0aIkLaO",
				"gap": 1.0760774742620924,
				"focus": 0.012027039962580007
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
					1.183029599680708,
					41.95271266376767
				]
			]
		},
		{
			"type": "arrow",
			"version": 371,
			"versionNonce": 2101992743,
			"isDeleted": false,
			"id": "7vp8C1qyiKu3LUzC1XIWu",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -136.30375195234726,
			"y": 231.5765405850318,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 186.88453254508707,
			"height": 1.5058122607466657,
			"seed": 111892201,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "5gzNiyV8"
				}
			],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "6jV65uDaRXHqh7u4F9LF3",
				"gap": 4.919561379631986,
				"focus": 0.06144712793168621
			},
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					186.88453254508707,
					-1.5058122607466657
				]
			]
		},
		{
			"type": "text",
			"version": 29,
			"versionNonce": 984296649,
			"isDeleted": false,
			"id": "5gzNiyV8",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": -60.43961067980372,
			"y": 218.82363445465847,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 35.15625,
			"height": 24,
			"seed": 1422417611,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "JAR",
			"rawText": "JAR",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "7vp8C1qyiKu3LUzC1XIWu",
			"originalText": "JAR",
			"lineHeight": 1.2
		},
		{
			"type": "text",
			"version": 705,
			"versionNonce": 2055315527,
			"isDeleted": false,
			"id": "Lcd8Sjm8",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 22.685934815596283,
			"y": -1126.039792672996,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 810.139404296875,
			"height": 100,
			"seed": 1338131499,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030573,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Create a DiSL Metamodel and use it to enforce structure of the DiSLClass?\n    - The code can be generated using a M2T transformation\n    - The M2T interface can be shared in the instrumentation-common module\n    - The M2T logic for emitting Java could be injected to the common interface",
			"rawText": "Create a DiSL Metamodel and use it to enforce structure of the DiSLClass?\n    - The code can be generated using a M2T transformation\n    - The M2T interface can be shared in the instrumentation-common module\n    - The M2T logic for emitting Java could be injected to the common interface",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Create a DiSL Metamodel and use it to enforce structure of the DiSLClass?\n    - The code can be generated using a M2T transformation\n    - The M2T interface can be shared in the instrumentation-common module\n    - The M2T logic for emitting Java could be injected to the common interface",
			"lineHeight": 1.25
		},
		{
			"type": "text",
			"version": 531,
			"versionNonce": 538749865,
			"isDeleted": false,
			"id": "bCKopXby",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 16.577571892529704,
			"y": -940.9391819742814,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 661.3394775390625,
			"height": 50,
			"seed": 2061403851,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 1,
			"text": "Create a Test Metamodel and use it for generating tests?\n    - Some sort of M2T could be reused from the instrumentation",
			"rawText": "Create a Test Metamodel and use it for generating tests?\n    - Some sort of M2T could be reused from the instrumentation",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "Create a Test Metamodel and use it for generating tests?\n    - Some sort of M2T could be reused from the instrumentation",
			"lineHeight": 1.25
		},
		{
			"type": "rectangle",
			"version": 690,
			"versionNonce": 2115239783,
			"isDeleted": false,
			"id": "lyjikFFH3YmCDuWMBhA49",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 383.09575454472724,
			"y": -339.69587384679994,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1538.878451759293,
			"height": 854.9576026945601,
			"seed": 1612867975,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 382,
			"versionNonce": 482037385,
			"isDeleted": false,
			"id": "eGA4HeB4",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 456.68321597623253,
			"y": -298.1285800259318,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 222.65625,
			"height": 24,
			"seed": 524918951,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "DiSLClass Metamodel",
			"rawText": "DiSLClass Metamodel",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "DiSLClass Metamodel",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 831,
			"versionNonce": 441285255,
			"isDeleted": false,
			"id": "6HcuIbislz5-do5d4zhUk",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 822.2860446700906,
			"y": -113.7153894664059,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 204.01398002812687,
			"height": 112.58660299607754,
			"seed": 1589441479,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "XnAlE70p"
				},
				{
					"id": "SUHtFZGV1Pk-XDNPnxSPk",
					"type": "arrow"
				},
				{
					"id": "9ZffXeguLYEYrpldZ_MY6",
					"type": "arrow"
				},
				{
					"id": "jbwKcpvZdpcYEolHxewnG",
					"type": "arrow"
				},
				{
					"id": "epdxVS6YEb7rhkOyyjvH2",
					"type": "arrow"
				}
			],
			"updated": 1750012030574,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 755,
			"versionNonce": 390239593,
			"isDeleted": false,
			"id": "XnAlE70p",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 871.558659684154,
			"y": -69.42208796836712,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 105.46875,
			"height": 24,
			"seed": 2036227461,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "DiSLClass",
			"rawText": "DiSLClass",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "6HcuIbislz5-do5d4zhUk",
			"originalText": "DiSLClass",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 554,
			"versionNonce": 1459700103,
			"isDeleted": false,
			"id": "m9s8_fouISSK4h7_tDbNL",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 445.75798048651063,
			"y": -28.15565139828209,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 181.6207563719618,
			"height": 106,
			"seed": 1989731047,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "qLoTDm4L"
				},
				{
					"id": "9ZffXeguLYEYrpldZ_MY6",
					"type": "arrow"
				},
				{
					"id": "08OoLG1oJRhWPFsaU0T6h",
					"type": "arrow"
				}
			],
			"updated": 1750012043394,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 492,
			"versionNonce": 121742217,
			"isDeleted": false,
			"id": "qLoTDm4L",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 454.5371086724915,
			"y": -11.155651398282089,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 164.0625,
			"height": 72,
			"seed": 1907261509,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012047052,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Package\n--------------\n- name",
			"rawText": "Package\n--------------\n- name",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "m9s8_fouISSK4h7_tDbNL",
			"originalText": "Package\n--------------\n- name",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 849,
			"versionNonce": 605537479,
			"isDeleted": false,
			"id": "a02uwIwbCZurI0qlo9mNN",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 448.6917623916115,
			"y": -190.62456424907737,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 186.13844283808703,
			"height": 103.80918438365197,
			"seed": 124382727,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "qyRNEAm9"
				},
				{
					"id": "jbwKcpvZdpcYEolHxewnG",
					"type": "arrow"
				},
				{
					"id": "08OoLG1oJRhWPFsaU0T6h",
					"type": "arrow"
				}
			],
			"updated": 1750012030574,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 822,
			"versionNonce": 499715881,
			"isDeleted": false,
			"id": "qyRNEAm9",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 506.60473381065503,
			"y": -150.71997205725137,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 70.3125,
			"height": 24,
			"seed": 1558508293,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Import",
			"rawText": "Import",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "a02uwIwbCZurI0qlo9mNN",
			"originalText": "Import",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 598,
			"versionNonce": 1248957415,
			"isDeleted": false,
			"id": "7s2UzpuKkJjRCmtzc_JEB",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1197.009480331063,
			"y": -109.72882200051458,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 261.76786071544575,
			"height": 106,
			"seed": 1814861095,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "7h5KDroU"
				},
				{
					"id": "JVhLoN_7sD657Oww_s9xV",
					"type": "arrow"
				},
				{
					"id": "xszlSyl3BNxF8heMRkZiY",
					"type": "arrow"
				},
				{
					"id": "g85mE5XTWKEVVs3Xli6-X",
					"type": "arrow"
				},
				{
					"id": "JVAQ4oP4dCR2MsvITXLbn",
					"type": "arrow"
				},
				{
					"id": "SUHtFZGV1Pk-XDNPnxSPk",
					"type": "arrow"
				},
				{
					"id": "pJscZpHcVzQj6rs53irSv",
					"type": "arrow"
				},
				{
					"id": "epdxVS6YEb7rhkOyyjvH2",
					"type": "arrow"
				}
			],
			"updated": 1750012030574,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 575,
			"versionNonce": 2114929161,
			"isDeleted": false,
			"id": "7h5KDroU",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1234.143410688786,
			"y": -104.72882200051458,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 187.5,
			"height": 96,
			"seed": 1370338757,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Instrumentation\nMethod\n----------------\n- name",
			"rawText": "Instrumentation Method\n----------------\n- name",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "7s2UzpuKkJjRCmtzc_JEB",
			"originalText": "Instrumentation Method\n----------------\n- name",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 728,
			"versionNonce": 520388359,
			"isDeleted": false,
			"id": "WeuloebsWA1RvatcKC1Yd",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 803.9440749549867,
			"y": 120.27875925348621,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 211.73866614613144,
			"height": 114.37136801973202,
			"seed": 1817008199,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "oBdnkh2N"
				},
				{
					"id": "JVAQ4oP4dCR2MsvITXLbn",
					"type": "arrow"
				},
				{
					"id": "g85mE5XTWKEVVs3Xli6-X",
					"type": "arrow"
				},
				{
					"id": "xszlSyl3BNxF8heMRkZiY",
					"type": "arrow"
				}
			],
			"updated": 1750012030574,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 712,
			"versionNonce": 550246633,
			"isDeleted": false,
			"id": "oBdnkh2N",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 816.0634080280524,
			"y": 141.46444326335222,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 187.5,
			"height": 72,
			"seed": 733268101,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Annotation\n----------------\n- activationTime",
			"rawText": "Annotation\n----------------\n- activationTime",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "WeuloebsWA1RvatcKC1Yd",
			"originalText": "Annotation\n----------------\n- activationTime",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 883,
			"versionNonce": 877463079,
			"isDeleted": false,
			"id": "oIkeAwSbdlHdJxpUVjHTw",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 634.5945537602302,
			"y": 355.03632037751163,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 220.1535543631271,
			"height": 112.28681824601063,
			"seed": 766031719,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "bHAl32JU"
				},
				{
					"id": "g85mE5XTWKEVVs3Xli6-X",
					"type": "arrow"
				}
			],
			"updated": 1750012030574,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 853,
			"versionNonce": 1200296905,
			"isDeleted": false,
			"id": "bHAl32JU",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 650.9213309417937,
			"y": 375.17972950051694,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 187.5,
			"height": 72,
			"seed": 1269181253,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Marker\n----------------\n- markerClass",
			"rawText": "Marker\n----------------\n- markerClass",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "oIkeAwSbdlHdJxpUVjHTw",
			"originalText": "Marker\n----------------\n- markerClass",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 914,
			"versionNonce": 2098466857,
			"isDeleted": false,
			"id": "3LHDcG7Fa1merGRZvHcf2",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 910.6023145424094,
			"y": 354.9680250615859,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 226.4514933056737,
			"height": 116.50020807324177,
			"seed": 1720398471,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "K8J6EW1f"
				},
				{
					"id": "xszlSyl3BNxF8heMRkZiY",
					"type": "arrow"
				}
			],
			"updated": 1750012063559,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 901,
			"versionNonce": 508699401,
			"isDeleted": false,
			"id": "K8J6EW1f",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 918.3593111952463,
			"y": 377.21812909820676,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 210.9375,
			"height": 72,
			"seed": 1462623749,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012063559,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Scope\n----------------\n- methodIdentifier",
			"rawText": "Scope\n----------------\n- methodIdentifier",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "3LHDcG7Fa1merGRZvHcf2",
			"originalText": "Scope\n----------------\n- methodIdentifier",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 854,
			"versionNonce": 1260447847,
			"isDeleted": false,
			"id": "ToFOEtK_uZpSKKwk5kAwP",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1450.39945010474,
			"y": 134.60039798056027,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 193.7376376653664,
			"height": 106,
			"seed": 1337018791,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "MBdKRmYS"
				},
				{
					"id": "JVhLoN_7sD657Oww_s9xV",
					"type": "arrow"
				},
				{
					"id": "AEqqBzMbPCFNJY8UQYDbh",
					"type": "arrow"
				},
				{
					"id": "Mqcaq-UoDtQGl6SySfJSz",
					"type": "arrow"
				},
				{
					"id": "Q2HYA7RgIsTOBKUH7ZUc1",
					"type": "arrow"
				}
			],
			"updated": 1750012030574,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 776,
			"versionNonce": 1707244617,
			"isDeleted": false,
			"id": "MBdKRmYS",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1459.3776439374233,
			"y": 151.60039798056027,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 175.78125,
			"height": 72,
			"seed": 2029510853,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012052610,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Exportable\n---------------\n- varIdentifier",
			"rawText": "Exportable\n---------------\n- varIdentifier",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "ToFOEtK_uZpSKKwk5kAwP",
			"originalText": "Exportable\n---------------\n- varIdentifier",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 1311,
			"versionNonce": 1905399017,
			"isDeleted": false,
			"id": "JVhLoN_7sD657Oww_s9xV",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1425.4154979793295,
			"y": 1.0607622621826067,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 80.15448687117191,
			"height": 117.73686531149269,
			"seed": 1447525575,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "k2N3E6NL"
				}
			],
			"updated": 1750012053842,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "7s2UzpuKkJjRCmtzc_JEB",
				"gap": 4.78958426269719,
				"focus": -0.3465010792951129
			},
			"endBinding": {
				"elementId": "ToFOEtK_uZpSKKwk5kAwP",
				"gap": 15.802770406884974,
				"focus": 0.0386773803304945
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
					80.15448687117191,
					117.73686531149269
				]
			]
		},
		{
			"type": "text",
			"version": 44,
			"versionNonce": 498588777,
			"isDeleted": false,
			"id": "k2N3E6NL",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1423.9547257612517,
			"y": 46.51308714456696,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 82.03125,
			"height": 24,
			"seed": 548418437,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "exports",
			"rawText": "exports",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "JVhLoN_7sD657Oww_s9xV",
			"originalText": "exports",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 2402,
			"versionNonce": 189379785,
			"isDeleted": false,
			"id": "xszlSyl3BNxF8heMRkZiY",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 954.6192596759661,
			"y": 241.39010890572854,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 79.52466210526359,
			"height": 105.36767419701374,
			"seed": 233458663,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "InvDj0Jn"
				}
			],
			"updated": 1750012063560,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "WeuloebsWA1RvatcKC1Yd",
				"gap": 6.7399816325103075,
				"focus": 0.019181547721148783
			},
			"endBinding": {
				"elementId": "3LHDcG7Fa1merGRZvHcf2",
				"gap": 8.210241958843653,
				"focus": 0.3847327256333408
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
					79.52466210526359,
					105.36767419701374
				]
			]
		},
		{
			"type": "text",
			"version": 49,
			"versionNonce": 1228040009,
			"isDeleted": false,
			"id": "InvDj0Jn",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 916.7125143249716,
			"y": 284.63074999924163,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 140.625,
			"height": 24,
			"seed": 1124259397,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "targetMethod",
			"rawText": "targetMethod",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "xszlSyl3BNxF8heMRkZiY",
			"originalText": "targetMethod",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 2445,
			"versionNonce": 1728138695,
			"isDeleted": false,
			"id": "g85mE5XTWKEVVs3Xli6-X",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 846.61874050405,
			"y": 247.4904158062218,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 71.16282996021891,
			"height": 87.45587283166901,
			"seed": 1655060231,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "kZV7HJ63"
				}
			],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "WeuloebsWA1RvatcKC1Yd",
				"gap": 12.840288533003559,
				"focus": 0.04077750037535953
			},
			"endBinding": {
				"elementId": "oIkeAwSbdlHdJxpUVjHTw",
				"gap": 20.090031739620827,
				"focus": -0.2006062324138537
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
					-71.16282996021891,
					87.45587283166901
				]
			]
		},
		{
			"type": "text",
			"version": 43,
			"versionNonce": 2128123433,
			"isDeleted": false,
			"id": "kZV7HJ63",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 775.8810755239406,
			"y": 279.2183522220563,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 70.3125,
			"height": 24,
			"seed": 429639941,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "marker",
			"rawText": "marker",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "g85mE5XTWKEVVs3Xli6-X",
			"originalText": "marker",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 1697,
			"versionNonce": 1765932263,
			"isDeleted": false,
			"id": "JVAQ4oP4dCR2MsvITXLbn",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1206.389047577657,
			"y": 3.2063679142445523,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 225.69375270594094,
			"height": 105.31606169303016,
			"seed": 184008231,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "DimuyQsQ"
				}
			],
			"updated": 1750012030574,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "7s2UzpuKkJjRCmtzc_JEB",
				"gap": 9.767405461483136,
				"focus": -0.0413053358141833
			},
			"endBinding": {
				"elementId": "WeuloebsWA1RvatcKC1Yd",
				"gap": 11.756329646211498,
				"focus": -0.3364946676192887
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
					-225.69375270594094,
					105.31606169303016
				]
			]
		},
		{
			"type": "text",
			"version": 77,
			"versionNonce": 1100343561,
			"isDeleted": false,
			"id": "DimuyQsQ",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1034.9484212246866,
			"y": 43.864398760759634,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 117.1875,
			"height": 24,
			"seed": 1663503301,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "annotation",
			"rawText": "annotation",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "JVAQ4oP4dCR2MsvITXLbn",
			"originalText": "annotation",
			"lineHeight": 1.2
		},
		{
			"type": "text",
			"version": 473,
			"versionNonce": 970967047,
			"isDeleted": false,
			"id": "lfIjSvUb",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1524.9090506877756,
			"y": 109.75507805629354,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 11.71875,
			"height": 24,
			"seed": 969630023,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "JVhLoN_7sD657Oww_s9xV",
					"type": "arrow"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "*",
			"rawText": "*",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "*",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 1016,
			"versionNonce": 719708137,
			"isDeleted": false,
			"id": "SUHtFZGV1Pk-XDNPnxSPk",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1034.2737095272505,
			"y": -55.410362569548646,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 142.5646250585146,
			"height": 1.5098790267587319,
			"seed": 276668519,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "b9rOm2t0"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "6HcuIbislz5-do5d4zhUk",
				"gap": 7.973684829033118,
				"focus": 0.01476179189993838
			},
			"endBinding": {
				"elementId": "I2NAW9hE",
				"gap": 6.2941410199161965,
				"focus": -1.5025936831977968
			},
			"lastCommittedPoint": null,
			"startArrowhead": "diamond",
			"endArrowhead": "arrow",
			"points": [
				[
					0,
					0
				],
				[
					142.5646250585146,
					1.5098790267587319
				]
			]
		},
		{
			"type": "text",
			"version": 42,
			"versionNonce": 1188507431,
			"isDeleted": false,
			"id": "b9rOm2t0",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1076.2591470565078,
			"y": -66.65542305616928,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 58.59375,
			"height": 24,
			"seed": 321160677,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "logic",
			"rawText": "logic",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "SUHtFZGV1Pk-XDNPnxSPk",
			"originalText": "logic",
			"lineHeight": 1.2
		},
		{
			"type": "text",
			"version": 325,
			"versionNonce": 416545481,
			"isDeleted": false,
			"id": "I2NAW9hE",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1179.4317159971333,
			"y": -85.18290458276238,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 11.71875,
			"height": 24,
			"seed": 1356528519,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "SUHtFZGV1Pk-XDNPnxSPk",
					"type": "arrow"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "*",
			"rawText": "*",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "*",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 1168,
			"versionNonce": 1393787945,
			"isDeleted": false,
			"id": "9ZffXeguLYEYrpldZ_MY6",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 816.270849468704,
			"y": -50.432638714081435,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 178.4503341193356,
			"height": 85.20810962489605,
			"seed": 1327493799,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "mzgZrip5"
				}
			],
			"updated": 1750012049073,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "6HcuIbislz5-do5d4zhUk",
				"gap": 6.015195201386518,
				"focus": 0.4246611138225677
			},
			"endBinding": {
				"elementId": "m9s8_fouISSK4h7_tDbNL",
				"gap": 10.441778490896013,
				"focus": 0.6047877194062723
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
					-178.4503341193356,
					85.20810962489605
				]
			]
		},
		{
			"type": "text",
			"version": 41,
			"versionNonce": 469481897,
			"isDeleted": false,
			"id": "mzgZrip5",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 686.8823254073717,
			"y": -20.2465344341539,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 82.03125,
			"height": 24,
			"seed": 1865331717,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "package",
			"rawText": "package",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "9ZffXeguLYEYrpldZ_MY6",
			"originalText": "package",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 1043,
			"versionNonce": 2113196391,
			"isDeleted": false,
			"id": "jbwKcpvZdpcYEolHxewnG",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 803.282456031214,
			"y": -59.990166939584654,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 155.9652822165599,
			"height": 73.78898348147008,
			"seed": 603522503,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [
				{
					"type": "text",
					"id": "xUlhX6dD"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "6HcuIbislz5-do5d4zhUk",
				"gap": 19.00358863887675,
				"focus": -0.5230161783082306
			},
			"endBinding": {
				"elementId": "g7hfsbPh",
				"gap": 4.1189000814752035,
				"focus": 1.2012376330247054
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
					-155.9652822165599,
					-73.78898348147008
				]
			]
		},
		{
			"type": "text",
			"version": 40,
			"versionNonce": 629709961,
			"isDeleted": false,
			"id": "xUlhX6dD",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 684.284189922934,
			"y": -108.88465868031969,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 82.03125,
			"height": 24,
			"seed": 456865477,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "imports",
			"rawText": "imports",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "jbwKcpvZdpcYEolHxewnG",
			"originalText": "imports",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 1507,
			"versionNonce": 1245973639,
			"isDeleted": false,
			"id": "epdxVS6YEb7rhkOyyjvH2",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1226.4870623597967,
			"y": -485.0552859863429,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 291.40724794212156,
			"height": 349.6144575286094,
			"seed": 511882471,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "UQ20XurYWnj-antpsMdNo",
				"gap": 8.613498447226902,
				"focus": -0.030487837793552082
			},
			"endBinding": {
				"elementId": "6HcuIbislz5-do5d4zhUk",
				"gap": 21.725438991327565,
				"focus": -0.11694128780078238
			},
			"lastCommittedPoint": null,
			"startArrowhead": "arrow",
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-247.46813823528817,
					211.43535719628437
				],
				[
					-291.40724794212156,
					349.6144575286094
				]
			]
		},
		{
			"type": "rectangle",
			"version": 747,
			"versionNonce": 1966482281,
			"isDeleted": false,
			"id": "UQ20XurYWnj-antpsMdNo",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1235.1005608070236,
			"y": -683.5642713632449,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 191.4683882881172,
			"height": 207.39918854404885,
			"seed": 1984528391,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"id": "epdxVS6YEb7rhkOyyjvH2",
					"type": "arrow"
				},
				{
					"type": "text",
					"id": "PGMcL9QY"
				},
				{
					"id": "pJscZpHcVzQj6rs53irSv",
					"type": "arrow"
				},
				{
					"id": "80rTLsm27RFxqysfsFUm7",
					"type": "arrow"
				},
				{
					"id": "0pyrPB8bsYBGyuhqSxhli",
					"type": "arrow"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 686,
			"versionNonce": 993977255,
			"isDeleted": false,
			"id": "PGMcL9QY",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1242.9441299510822,
			"y": -627.8646770912204,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 175.78125,
			"height": 96,
			"seed": 1485082853,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Metaclass\n---------------\n-\n+ visit()",
			"rawText": "Metaclass\n----------------\n+ visit()",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "UQ20XurYWnj-antpsMdNo",
			"originalText": "Metaclass\n----------------\n+ visit()",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 1333,
			"versionNonce": 1196412489,
			"isDeleted": false,
			"id": "pJscZpHcVzQj6rs53irSv",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1318.5046784539293,
			"y": -119.46806413243144,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 1.9788401715793498,
			"height": 352.05392556454217,
			"seed": 1028253479,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "7s2UzpuKkJjRCmtzc_JEB",
				"gap": 9.739242131916853,
				"focus": -0.06894699452887318
			},
			"endBinding": {
				"elementId": "UQ20XurYWnj-antpsMdNo",
				"gap": 4.643093122222382,
				"focus": 0.15488317369333976
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
					-1.9788401715793498,
					-352.05392556454217
				]
			]
		},
		{
			"type": "arrow",
			"version": 1162,
			"versionNonce": 1896994503,
			"isDeleted": false,
			"id": "80rTLsm27RFxqysfsFUm7",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1397.378966885687,
			"y": -471.3631713479988,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 165.235967146863,
			"height": 513.6929507476602,
			"seed": 300968519,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "UQ20XurYWnj-antpsMdNo",
				"gap": 4.801911471197201,
				"focus": 0.05519350287985601
			},
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": "arrow",
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					165.235967146863,
					236.4340948327963
				],
				[
					93.71250338786194,
					513.6929507476602
				]
			]
		},
		{
			"type": "arrow",
			"version": 933,
			"versionNonce": 1118903593,
			"isDeleted": false,
			"id": "0pyrPB8bsYBGyuhqSxhli",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1225.708013197655,
			"y": -565.5754195493869,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 473.82785382035127,
			"height": 429.25658165688924,
			"seed": 286886247,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "UQ20XurYWnj-antpsMdNo",
				"gap": 9.392547609368648,
				"focus": 0.37925636190420947
			},
			"endBinding": null,
			"lastCommittedPoint": null,
			"startArrowhead": "arrow",
			"endArrowhead": null,
			"points": [
				[
					0,
					0
				],
				[
					-384.8824855205544,
					299.86839045762986
				],
				[
					-473.82785382035127,
					429.25658165688924
				]
			]
		},
		{
			"type": "arrow",
			"version": 1123,
			"versionNonce": 599608809,
			"isDeleted": false,
			"id": "08OoLG1oJRhWPFsaU0T6h",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 548.0875956891605,
			"y": -81.12615501302494,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 2.1258857781665483,
			"height": 51.854671239692735,
			"seed": 135382151,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012049073,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "a02uwIwbCZurI0qlo9mNN",
				"gap": 5.689224852400443,
				"focus": -0.08897988699866209
			},
			"endBinding": {
				"elementId": "m9s8_fouISSK4h7_tDbNL",
				"gap": 1.1158323750501182,
				"focus": 0.0771619300983562
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
					-2.1258857781665483,
					51.854671239692735
				]
			]
		},
		{
			"type": "rectangle",
			"version": 936,
			"versionNonce": 650481673,
			"isDeleted": false,
			"id": "SQ9rRJ55G5pU70CkAZoYF",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 502.92126497687605,
			"y": -683.6572329215776,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 257.8881015044217,
			"height": 212.03862339834996,
			"seed": 1721080743,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "H7fy2xLa"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 853,
			"versionNonce": 1042419975,
			"isDeleted": false,
			"id": "H7fy2xLa",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 538.1153157290869,
			"y": -625.6379212224026,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 187.5,
			"height": 96,
			"seed": 690872613,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Model\n----------------\n- dislClass\n+ visit()",
			"rawText": "Model\n----------------\n- dislClass\n+ visit()",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "SQ9rRJ55G5pU70CkAZoYF",
			"originalText": "Model\n----------------\n- dislClass\n+ visit()",
			"lineHeight": 1.2
		},
		{
			"type": "text",
			"version": 186,
			"versionNonce": 965256937,
			"isDeleted": false,
			"id": "g7hfsbPh",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 644.9664440666829,
			"y": -162.89805050252994,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 11.71875,
			"height": 24,
			"seed": 1892781767,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [
				{
					"id": "jbwKcpvZdpcYEolHxewnG",
					"type": "arrow"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "*",
			"rawText": "*",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "*",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 790,
			"versionNonce": 2112378919,
			"isDeleted": false,
			"id": "TebJkwXrCtS8RmzJhZoYS",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1246.2237359581513,
			"y": 365.03174856374994,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 194.065309368928,
			"height": 107.39962964608185,
			"seed": 828513767,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "RAUFJIgC"
				},
				{
					"id": "AEqqBzMbPCFNJY8UQYDbh",
					"type": "arrow"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 797,
			"versionNonce": 1971708361,
			"isDeleted": false,
			"id": "RAUFJIgC",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1296.3813906426153,
			"y": 406.73156338679087,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 93.75,
			"height": 24,
			"seed": 1198970693,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Argument",
			"rawText": "Argument",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "TebJkwXrCtS8RmzJhZoYS",
			"originalText": "Argument",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 784,
			"versionNonce": 2006498119,
			"isDeleted": false,
			"id": "0BSm204t-tbDvfjhvxoPu",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1462.3761912615892,
			"y": 364.5687724617958,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 194.065309368928,
			"height": 107.39962964608185,
			"seed": 726911239,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "YXjV3ieV"
				},
				{
					"id": "Mqcaq-UoDtQGl6SySfJSz",
					"type": "arrow"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 786,
			"versionNonce": 2010693801,
			"isDeleted": false,
			"id": "YXjV3ieV",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1530.1119709460531,
			"y": 406.2685872848367,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 58.59375,
			"height": 24,
			"seed": 2143521285,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Field",
			"rawText": "Field",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "0BSm204t-tbDvfjhvxoPu",
			"originalText": "Field",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 766,
			"versionNonce": 529513063,
			"isDeleted": false,
			"id": "C72P0yBkPclYDtBczAhrt",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1675.4546679191508,
			"y": 364.0256047712494,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 194.065309368928,
			"height": 107.39962964608185,
			"seed": 1277366311,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "WqyLX9QA"
				},
				{
					"id": "Q2HYA7RgIsTOBKUH7ZUc1",
					"type": "arrow"
				}
			],
			"updated": 1750012030575,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 765,
			"versionNonce": 774685577,
			"isDeleted": false,
			"id": "WqyLX9QA",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1725.6123226036148,
			"y": 405.7254195942903,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 93.75,
			"height": 24,
			"seed": 554863813,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030575,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Variable",
			"rawText": "Variable",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "C72P0yBkPclYDtBczAhrt",
			"originalText": "Variable",
			"lineHeight": 1.2
		},
		{
			"type": "arrow",
			"version": 772,
			"versionNonce": 316583593,
			"isDeleted": false,
			"id": "AEqqBzMbPCFNJY8UQYDbh",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1412.9229285588158,
			"y": 350.8909870096088,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 86.58106753843799,
			"height": 102.90317532280415,
			"seed": 498889543,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012053842,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "TebJkwXrCtS8RmzJhZoYS",
				"gap": 14.140761554141136,
				"focus": 0.09085283043696135
			},
			"endBinding": {
				"elementId": "ToFOEtK_uZpSKKwk5kAwP",
				"gap": 7.387413706244388,
				"focus": -0.0215232145043588
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle_outline",
			"points": [
				[
					0,
					0
				],
				[
					86.58106753843799,
					-102.90317532280415
				]
			]
		},
		{
			"type": "arrow",
			"version": 635,
			"versionNonce": 228579433,
			"isDeleted": false,
			"id": "Mqcaq-UoDtQGl6SySfJSz",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1561.8638572917057,
			"y": 347.360425114063,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 1.5288708120278898,
			"height": 99.24619851554053,
			"seed": 947679847,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012053842,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "0BSm204t-tbDvfjhvxoPu",
				"gap": 17.208347347732797,
				"focus": 0.03619958966411002
			},
			"endBinding": {
				"elementId": "ToFOEtK_uZpSKKwk5kAwP",
				"gap": 7.51382861796219,
				"focus": -0.12422048588843457
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle_outline",
			"points": [
				[
					0,
					0
				],
				[
					-1.5288708120278898,
					-99.24619851554053
				]
			]
		},
		{
			"type": "arrow",
			"version": 613,
			"versionNonce": 1509003817,
			"isDeleted": false,
			"id": "Q2HYA7RgIsTOBKUH7ZUc1",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1741.739282873931,
			"y": 350.46638206788543,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 132.33609213282148,
			"height": 104.8456081974823,
			"seed": 1966050695,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012053842,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "C72P0yBkPclYDtBczAhrt",
				"gap": 13.559222703363957,
				"focus": 0.326320464640924
			},
			"endBinding": {
				"elementId": "ToFOEtK_uZpSKKwk5kAwP",
				"gap": 5.020375889842853,
				"focus": 0.06776997729987175
			},
			"lastCommittedPoint": null,
			"startArrowhead": null,
			"endArrowhead": "triangle_outline",
			"points": [
				[
					0,
					0
				],
				[
					-132.33609213282148,
					-104.8456081974823
				]
			]
		},
		{
			"type": "rectangle",
			"version": 723,
			"versionNonce": 1414935881,
			"isDeleted": false,
			"id": "e0GXbPrMogUTWvOKCc926",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -127.3335005119144,
			"y": -1854.218598686673,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 702.4069312848604,
			"height": 571.6994988740456,
			"seed": 1816301671,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"id": "LincC3KJ0spbw9egVEPBm",
					"type": "arrow"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 632,
			"versionNonce": 1799970759,
			"isDeleted": false,
			"id": "JjpNZSnW",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -100.04888989919266,
			"y": -1818.2595029813724,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 199.21875,
			"height": 24,
			"seed": 1182919559,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "DiSL Class Layout",
			"rawText": "DiSL Class Layout",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "DiSL Class Layout",
			"lineHeight": 1.2
		},
		{
			"type": "text",
			"version": 707,
			"versionNonce": 1347593257,
			"isDeleted": false,
			"id": "N1TyDhs9",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": -87.41381199962518,
			"y": -1710.1958077799413,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 621.09375,
			"height": 336,
			"seed": 1382605479,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "package %s;\n\n%s\n\npublic class %s {\n\n  @After(marker = %s, scope = \"%s\")\n  public static void %s(DynamicContext di) {\n      int a = di.getLocalVariableValue(0, int.class);\n      int b = di.getLocalVariableValue(1, int.class);\n      System.out.println(\"disl: a=\" + a);\n      System.out.println(\"disl: b=\" + b);\n  }\n}",
			"rawText": "package %s;\n\n%s\n\npublic class %s {\n\n  @After(marker = %s, scope = \"%s\")\n  public static void %s(DynamicContext di) {\n      int a = di.getLocalVariableValue(0, int.class);\n      int b = di.getLocalVariableValue(1, int.class);\n      System.out.println(\"disl: a=\" + a);\n      System.out.println(\"disl: b=\" + b);\n  }\n}",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "package %s;\n\n%s\n\npublic class %s {\n\n  @After(marker = %s, scope = \"%s\")\n  public static void %s(DynamicContext di) {\n      int a = di.getLocalVariableValue(0, int.class);\n      int b = di.getLocalVariableValue(1, int.class);\n      System.out.println(\"disl: a=\" + a);\n      System.out.println(\"disl: b=\" + b);\n  }\n}",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 656,
			"versionNonce": 246927079,
			"isDeleted": false,
			"id": "iyy-urdvRaoRrHOKu5S7a",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 818.6700234274954,
			"y": -1852.7579136643658,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 501.2913950025726,
			"height": 77.82345407099206,
			"seed": 816051655,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 524,
			"versionNonce": 882784009,
			"isDeleted": false,
			"id": "pzQFrZ9l",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 831.7479449515463,
			"y": -1827.305528497754,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 82.03125,
			"height": 24,
			"seed": 944743655,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "package",
			"rawText": "package",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "package",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 495,
			"versionNonce": 348871175,
			"isDeleted": false,
			"id": "zFauEsksu-tF0tYjYx0lg",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 929.7113068114099,
			"y": -1842.5486385661288,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 229.61961549807074,
			"height": 58.204287830755675,
			"seed": 687370247,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "unPkX3IS"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 482,
			"versionNonce": 1941204457,
			"isDeleted": false,
			"id": "unPkX3IS",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 974.2086145604453,
			"y": -1825.446494650751,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 140.625,
			"height": 24,
			"seed": 1754851851,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "PACKAGE_NAME",
			"rawText": "PACKAGE_NAME",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "zFauEsksu-tF0tYjYx0lg",
			"originalText": "PACKAGE_NAME",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 643,
			"versionNonce": 578847015,
			"isDeleted": false,
			"id": "CMkDnDW52ufSR-CStghbS",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 816.2490320171751,
			"y": -1760.6780177224123,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 508.25288728432315,
			"height": 99.10990684852459,
			"seed": 479909671,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "6sa9bsE8"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 567,
			"versionNonce": 614516937,
			"isDeleted": false,
			"id": "6sa9bsE8",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1029.3598506593366,
			"y": -1723.12306429815,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 82.03125,
			"height": 24,
			"seed": 1084760907,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "Imports",
			"rawText": "Imports",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "CMkDnDW52ufSR-CStghbS",
			"originalText": "Imports",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 788,
			"versionNonce": 155436103,
			"isDeleted": false,
			"id": "W9IQbuOlEjsSNvefu_Mjo",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 817.1763174064113,
			"y": -1644.0775899114042,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 513.2090678129974,
			"height": 433.2669240181397,
			"seed": 967006791,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"id": "LincC3KJ0spbw9egVEPBm",
					"type": "arrow"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 498,
			"versionNonce": 1895745449,
			"isDeleted": false,
			"id": "FEnNVw67",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 839.7052412473536,
			"y": -1619.4839715092075,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 140.625,
			"height": 24,
			"seed": 619608423,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "public class",
			"rawText": "public class",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "public class",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 616,
			"versionNonce": 955360103,
			"isDeleted": false,
			"id": "l45kBXgpnjjCkoKnOBARU",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 995.278719595346,
			"y": -1627.587440871299,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 283.9538061608191,
			"height": 34.64309026508295,
			"seed": 868791431,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "CyxhAhUr"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 550,
			"versionNonce": 1123674761,
			"isDeleted": false,
			"id": "CyxhAhUr",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1078.6618726757556,
			"y": -1622.2658957387575,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 117.1875,
			"height": 24,
			"seed": 1437771211,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "CLASS_NAME",
			"rawText": "CLASS_NAME",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "l45kBXgpnjjCkoKnOBARU",
			"originalText": "CLASS_NAME",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 1091,
			"versionNonce": 1909890695,
			"isDeleted": false,
			"id": "scQ9Dl37Zt7gidN6H6dCb",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 838.9630041944934,
			"y": -1577.8109438934503,
			"strokeColor": "#e03131",
			"backgroundColor": "transparent",
			"width": 473.53678404199536,
			"height": 342.75729856935334,
			"seed": 1575914407,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 483,
			"versionNonce": 860729705,
			"isDeleted": false,
			"id": "14T2GbeW",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 854.5874865583414,
			"y": -1497.7446468366707,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 140.625,
			"height": 24,
			"seed": 1871478471,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "static void ",
			"rawText": "static void ",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "static void ",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 590,
			"versionNonce": 917452199,
			"isDeleted": false,
			"id": "BU8XLnNSENLCwVMZsVKhh",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 999.0161267543435,
			"y": -1502.7190989986302,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 284.324346144758,
			"height": 34,
			"seed": 2067729895,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "ngq5TkdC"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 578,
			"versionNonce": 927308873,
			"isDeleted": false,
			"id": "ngq5TkdC",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1076.7251748267227,
			"y": -1497.7190989986302,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 128.90625,
			"height": 24,
			"seed": 36169803,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "INST_METHOD",
			"rawText": "INST_METHOD",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "BU8XLnNSENLCwVMZsVKhh",
			"originalText": "INST_METHOD",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 718,
			"versionNonce": 2124933319,
			"isDeleted": false,
			"id": "JD2dRKtBGR2EzN6PfiPMJ",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 851.4950126247809,
			"y": -1563.9930212213394,
			"strokeColor": "#2f9e44",
			"backgroundColor": "transparent",
			"width": 431.05066874503564,
			"height": 50.96872104972341,
			"seed": 202553607,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "rectangle",
			"version": 588,
			"versionNonce": 247278377,
			"isDeleted": false,
			"id": "U3wdWicIRdOPutFv7yOeX",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 863.2254011940308,
			"y": -1556.2093054417433,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 157.117774621462,
			"height": 37.3797179926305,
			"seed": 394526759,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "lUbAUeQj"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 520,
			"versionNonce": 660420583,
			"isDeleted": false,
			"id": "lUbAUeQj",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 883.1905385047619,
			"y": -1549.519446445428,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 117.1875,
			"height": 24,
			"seed": 635971627,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "ANNOTATION",
			"rawText": "ANNOTATION",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "U3wdWicIRdOPutFv7yOeX",
			"originalText": "ANNOTATION",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 642,
			"versionNonce": 1914691081,
			"isDeleted": false,
			"id": "y4OHhKjE1alMA3RxweEFH",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1028.848621109863,
			"y": -1555.7981936928209,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 126.11538084439871,
			"height": 35,
			"seed": 796877639,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "Bb4Ac1DY"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 470,
			"versionNonce": 651928327,
			"isDeleted": false,
			"id": "Bb4Ac1DY",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1056.7500615320623,
			"y": -1550.2981936928209,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 70.3125,
			"height": 24,
			"seed": 1939233131,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "MARKER",
			"rawText": "MARKER",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "y4OHhKjE1alMA3RxweEFH",
			"originalText": "MARKER",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 534,
			"versionNonce": 6894825,
			"isDeleted": false,
			"id": "CMu2i0Tlwh9X54_t3pkdz",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 1167.2242678887892,
			"y": -1554.8252292203715,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 101.6222564252721,
			"height": 35,
			"seed": 1952539239,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "qxrWXPlz"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 467,
			"versionNonce": 2107615783,
			"isDeleted": false,
			"id": "qxrWXPlz",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1188.7385211014252,
			"y": -1549.3252292203715,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 58.59375,
			"height": 24,
			"seed": 970537643,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "SCOPE",
			"rawText": "SCOPE",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "CMu2i0Tlwh9X54_t3pkdz",
			"originalText": "SCOPE",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 646,
			"versionNonce": 1831120841,
			"isDeleted": false,
			"id": "hpbC0U063xKho7pc0Xpg9",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 854.9296672476389,
			"y": -1459.335083197073,
			"strokeColor": "#2f9e44",
			"backgroundColor": "transparent",
			"width": 431.0049896618225,
			"height": 205.7746011664788,
			"seed": 1190559111,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "arrow",
			"version": 593,
			"versionNonce": 1754647879,
			"isDeleted": false,
			"id": "LincC3KJ0spbw9egVEPBm",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "dashed",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 586.6659644236988,
			"y": -1565.7286380561866,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 205.06489129200168,
			"height": 0.23744636590913615,
			"seed": 981152935,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 2
			},
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"startBinding": {
				"elementId": "e0GXbPrMogUTWvOKCc926",
				"focus": 0.010690750048770759,
				"gap": 11.592533650752785
			},
			"endBinding": {
				"elementId": "W9IQbuOlEjsSNvefu_Mjo",
				"focus": 0.6400597849643788,
				"gap": 25.445461690710772
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
					205.06489129200168,
					-0.23744636590913615
				]
			]
		},
		{
			"type": "text",
			"version": 143,
			"versionNonce": 1852690089,
			"isDeleted": false,
			"id": "wK0A1Qxj",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 880.0410362217549,
			"y": -1442.7520614228283,
			"strokeColor": "#2f9e44",
			"backgroundColor": "transparent",
			"width": 117.1875,
			"height": 24,
			"seed": 1184630727,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "INST_LOGIC",
			"rawText": "INST_LOGIC",
			"textAlign": "left",
			"verticalAlign": "top",
			"containerId": null,
			"originalText": "INST_LOGIC",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 1230,
			"versionNonce": 1394248807,
			"isDeleted": false,
			"id": "2DKZ3nngttwGKwKtsTC-u",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 861.650651350054,
			"y": -1407.800285289154,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 408.01575642275156,
			"height": 38.68629691316938,
			"seed": 963999463,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "qHGqlOKQ"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1323,
			"versionNonce": 1794118025,
			"isDeleted": false,
			"id": "qHGqlOKQ",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1012.9241545614298,
			"y": -1400.4571368325694,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 105.46875,
			"height": 24,
			"seed": 1380462539,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030576,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "ARGUMENTS",
			"rawText": "ARGUMENTS",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "2DKZ3nngttwGKwKtsTC-u",
			"originalText": "ARGUMENTS",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 1224,
			"versionNonce": 150628231,
			"isDeleted": false,
			"id": "JpmlNMQQVsYwR3mBxhryU",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 862.9100859756786,
			"y": -1358.2693678406913,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 408.8748329138988,
			"height": 38.68629691316938,
			"seed": 1771307527,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "dScp8SY6"
				}
			],
			"updated": 1750012030576,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1249,
			"versionNonce": 1743190121,
			"isDeleted": false,
			"id": "dScp8SY6",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1014.6131274326281,
			"y": -1350.9262193841066,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 105.46875,
			"height": 24,
			"seed": 873527563,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030577,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "VARIABLES",
			"rawText": "VARIABLES",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "JpmlNMQQVsYwR3mBxhryU",
			"originalText": "VARIABLES",
			"lineHeight": 1.2
		},
		{
			"type": "rectangle",
			"version": 1338,
			"versionNonce": 678551207,
			"isDeleted": false,
			"id": "IYtGrJicZWdC3CgPo1T9j",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 0,
			"opacity": 100,
			"angle": 0,
			"x": 865.7415439259198,
			"y": -1310.3097761123609,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 406.66796377612525,
			"height": 37.09592659842293,
			"seed": 670312743,
			"groupIds": [],
			"frameId": null,
			"roundness": {
				"type": 3
			},
			"boundElements": [
				{
					"type": "text",
					"id": "KFg8fMyF"
				}
			],
			"updated": 1750012030577,
			"link": null,
			"locked": false
		},
		{
			"type": "text",
			"version": 1356,
			"versionNonce": 924713801,
			"isDeleted": false,
			"id": "KFg8fMyF",
			"fillStyle": "solid",
			"strokeWidth": 2,
			"strokeStyle": "solid",
			"roughness": 1,
			"opacity": 100,
			"angle": 0,
			"x": 1033.9192758139825,
			"y": -1303.7618128131494,
			"strokeColor": "#1e1e1e",
			"backgroundColor": "transparent",
			"width": 70.3125,
			"height": 24,
			"seed": 178811467,
			"groupIds": [],
			"frameId": null,
			"roundness": null,
			"boundElements": [],
			"updated": 1750012030577,
			"link": null,
			"locked": false,
			"fontSize": 20,
			"fontFamily": 3,
			"text": "FIELDS",
			"rawText": "FIELDS",
			"textAlign": "center",
			"verticalAlign": "middle",
			"containerId": "IYtGrJicZWdC3CgPo1T9j",
			"originalText": "FIELDS",
			"lineHeight": 1.2
		}
	],
	"appState": {
		"theme": "light",
		"viewBackgroundColor": "#ffffff",
		"currentItemStrokeColor": "#1e1e1e",
		"currentItemBackgroundColor": "transparent",
		"currentItemFillStyle": "solid",
		"currentItemStrokeWidth": 2,
		"currentItemStrokeStyle": "solid",
		"currentItemRoughness": 0,
		"currentItemOpacity": 100,
		"currentItemFontFamily": 3,
		"currentItemFontSize": 20,
		"currentItemTextAlign": "left",
		"currentItemStartArrowhead": null,
		"currentItemEndArrowhead": "arrow",
		"scrollX": 557.0045631635885,
		"scrollY": 1100.8269873709014,
		"zoom": {
			"value": 0.7020375130326418
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