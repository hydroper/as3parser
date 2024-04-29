# Special cases

## Strict

Strict mode (default) or ECMAScript mode.

## Infer types

Optional infer types option introduced by Apache Royale.

Infers for initializers and method result types.

## Reference lookups in source path list

Similiar to `--source-path` in the MXML compiler, and optional main class file.

This form of reference lookup does not allow zero or more than one package definition in a file, and the definition at the package must match exactly the source path in the source tree.

## Reference lookups in included sources list

Similiar to `--include-sources` (recursive directory or specific file) and `--doc-sources` in the MXML compiler.

This form of reference lookup allows multiple package definitions within a file.

## Embed meta-data

HARMAN introduced additional forms of embedding data with encryption.

## Vector Data Type

[Information](https://github.com/hydroper/as3parser/blob/0.3/docs/verifier/vector.md)

## Namespaces

[Information](https://github.com/hydroper/as3parser/blob/0.3/docs/verifier/Type/kinds/namespace.md)

Remember of `static protected`.

## Event meta-data

Used by MXML components for event handling. For example, consider the `creationComplete="someCall()"` attribute in a XML file, containing zero or more directives.

## Data binding

Certain property attributes in MXML components may have values containing at most one braces group, containing a single expression that locates a bindable property.

Example from Apache Royale:

```xml
<j:TextInput id="databinding_ti">
    <j:beads>
        <j:TextPrompt prompt="Using databinding"/>
    </j:beads>
</j:TextInput>

<j:Label text="The TextInput field text value is: {databinding_ti.text}"/>
```

What about string literals, in case you want to escape the braces? Supporting them might not be an issue.

## Royale meta-data

[See all Royale meta-data here](https://apache.github.io/royale-docs/features/as3/metadata)

## Inline constants

Inline constants are replaced by their compile-time constant value.

```
CONFIG::DEBUG
```

## Scope

```as3
package {
    import flash.display.Sprite;
    // Definitions may be nested in blocks.
    {
        public class Main extends Sprite {
            {
                protected var xy: *
            }
        }
    }
}
var x: Number
{
    class Example {
        function f(): void {
            // "x" is visible at this scope.
            x
        }
    }
}
// "Example" is visible at this scope.
Example
```

## Float class

[Information](https://github.com/airsdk/Adobe-Runtime-Support/discussions/3081#discussioncomment-9091556)

## Package Shadowing

Properties from imported packages, when fully qualified, shadow variable names in scope.

## Importing ActionScript 3 components in MXML

* Use a `xmlns` prefix assigned to the full package name with a trailling `.*` sequence, as in `xmlns:fb="foo.bar.*"`.
* Use the `import` directive in a `fx:Script` tag and refer lexically to a component in the MXML.

## Dynamic

* The `Object` and `*` data types are fully dynamic, thus they may be mixed freely in operations, such as addition, property and query operators.
* There are `dynamic` classes, which allow for dynamic property operations.
