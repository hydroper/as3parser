type T1 = undefined;
type T2 = T?;
type T3 = T!;
type T4 = ?T;
type T5 =
    | void
    | *
    | String;
type T6 = (T);
type T7 = (v: T) => void;
type T8 = {} & {};