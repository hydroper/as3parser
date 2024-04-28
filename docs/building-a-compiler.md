# Building a Compiler

An ActionScript compiler does more than just parsing, including verification, and SWF and SWC processing.

An ActionScript compiler handles three source file formats: ActionScript 3, MXML, and CSS.

An ActionScript compiler outputs several problems, constructs a flow graph for every activation, and attaches meaning to tree nodes.

This project itself is not a compiler, but it's designated to facilitate writing one, parsing the three file formats mentioned.

## Compiler options

Attach compiler options to a compilation unit by defining your own options data type and storing a reference of it by calling `compilation_unit.set_compiler_options(Some(options))` where `options: Rc<dyn Any>`.

The parser project leaves the compiler options to be defined as part of the compiler.

## SWF

The Ruffle player contains a [`swf` package](https://github.com/ruffle-rs/ruffle/tree/master/swf) that reads and writes SWF from or into structures.

## Domain Memory Operations

Detect [domain memory operations](https://obtw.wordpress.com/2013/04/03/making-bytearray-faster) to generate optimized AVM2 instructions.

## Project Configuration

It is interesting to consider allowing project configurations in a dependency managed manner. Something similiar to `asconfig.json` from the Visual Studio AS&MXML extension, but in a different way to allow dependencies that include their own ActionScript 3 configuration (including options such as `strict`, and `inferTypes`).

## Special Cases

Here is the [Special Cases](compilers/special-cases.md) document.

Here is the [IDE](compilers/ide.md) document.