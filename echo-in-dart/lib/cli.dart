//This file contains cli handling for echo compiler

import 'package:echo/compiler.dart' as compiler;

//echo version
const version = "1.0.0";

//Starts the compiler
void run(List<String> args) {
  if (args.isEmpty) {
    print('echo $version');
  } else if (args.length == 1) {
    compiler.compile(args[0]);
  } else {
    print('Too many args!');
  }
}
