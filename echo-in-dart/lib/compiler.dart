//This file contains echo's compiling functionality

import 'dart:io';

//Compiles the echo source files
void compile(String fileName) async {
  var code = await getSourceCode(fileName);
  print('Source code:\n$code');
}

//Reads code from source file and returns it as a string
Future<String> getSourceCode(String fileName) async {
  if (!fileName.endsWith(".echo")) {
    print('Not a valid source file!');
    exit(1);
  }

  //Variable to store source code
  var code = '';
  //Source file
  var file = File(fileName);
  if (file.existsSync()) {
    code = await file.readAsString();
    return code;
  } else {
    print('$fileName does not exist!');
    exit(1);
  }
}
