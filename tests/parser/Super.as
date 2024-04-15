super() // ERROR
class C2 extends C1 {
    function C2() {
        if (true) {
            super()
        }
    }
    override protected function m(): void {
        super.m()
        super(this).m()
    }
}