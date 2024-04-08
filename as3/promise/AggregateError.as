package {
    public class AggregateError extends Error {
        public var errors: Array;

        public function AggregateError(errors: Array, message: String = "") {
            super(message);
            this.errors = errors.slice(0);
        }
    }
}