# Vector type

The `Vector` class is treated in a special way for compatibility. The program or an external library defines the `__AS3__.vec.Vector` class and the verifier automatically turns it into a generic class with a single type parameter.

## Code generation

An ActionScript 3 compiler should generate a multiname for a `Vector` type with type arguments as follows:

```plain
TypeName(QName(PackageNamespace("__AS3__.vec"),"Vector")<QName(PackageNamespace(""),"Number")>)
```

An ActionScript 3 compiler should generate a class for a `Vector` type with type arguments as follows:

```plain
getlex QName(PackageNamespace("__AS3__.vec"),"Vector")
getlex QName(PackageNamespace(""),"Number")
```