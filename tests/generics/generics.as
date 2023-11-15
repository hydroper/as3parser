class C.<T: C1 + C2> {
    function f(): void where T: C3 + C4 {}

    function f2.<T = void>(): void {}
}