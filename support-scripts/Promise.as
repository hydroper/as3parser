package {
    import flash.utils.setTimeout;
    import org.as3.lang.internals.*;

    /**
     * The Promise object represents the eventual completion (or failure)
     * of an asynchronous operation and its resulting value.
     * 
     * For more information, consult [developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise).
     *
     * # Examples
     * 
     * ```
     * const promise = new Promise.<void>((resolve, reject) => {
     *     //
     * });
     * 
     * promise
     *     .then(() => {})
     *     .catch(error => {})
     *     .finally(() => {});
     * ```
     */
    public final class Promise.<T> {
        // Implementation based on
        // https://github.com/taylorhakes/promise-polyfill

        private var m_state:Number = 0;
        private var m_handled:Boolean = false;
        private var m_value:* = undefined;
        private var m_deferreds:Vector.<PromiseHandler> = new Vector.<PromiseHandler>;

        private static function bindFunction(fn:Function, thisArg:*):Function {
            return function(...argumentsList):void {
                fn.apply(thisArg, argumentsList);
            };
        }

        public function Promise(fn: Function) {
            doResolve(fn, this);
        }

        private static function handle(self:Promise.<T>, deferred:PromiseHandler):void {
            while (self.m_state === 3) {
                self = self.m_value;
            }
            if (self.m_state === 0) {
                self.m_deferreds.push(deferred);
                return;
            }
            self.m_handled = true;
            Promise.<T>._immediateFn(function():void {
                var cb:Function = self.m_state === 1 ? deferred.onFulfilled : deferred.onRejected;
                if (cb === null) {
                    (self.m_state === 1 ? Promise.<T>.privateResolve : Promise.<T>.privateReject)(deferred.promise, self.m_value);
                    return;
                }
                var ret:* = undefined;
                try {
                    ret = cb(self.m_value);
                }
                catch (e:*) {
                    Promise.<T>.privateReject(deferred.promise, e);
                    return;
                }
                Promise.<T>.privateResolve(deferred.promise, ret);
            });
        }

        private static function privateResolve(self:Promise.<T>, newValue:*):void {
            try {
                // Promise Resolution Procedure: https://github.com/promises-aplus/promises-spec#the-promise-resolution-procedure
                if (newValue === self) {
                    throw new TypeError('A promise cannot be resolved with itself.');
                }
                if (newValue is Promise) {
                    self.m_state = 3;
                    self.m_value = newValue;
                    Promise.<T>.finale(self);
                    return;
                }
                self.m_state = 1;
                self.m_value = newValue;
                Promise.<T>.finale(self);
            }
            catch (e:*) {
                Promise.<T>.privateReject(self, e);
            }
        }

        private static function privateReject(self:Promise.<T>, newValue:*):void {
            self.m_state = 2;
            self.m_value = newValue;
            Promise.<T>.finale(self);
        }

        private static function finale(self:Promise.<T>):void {
            if (self.m_state === 2 && self.m_deferreds.length === 0) {
                Promise.<T>._immediateFn(function():void {
                    if (!self.m_handled) {
                        Promise.<T>._unhandledRejectionFn(self.m_value);
                    }
                });
            }

            for (var i:Number = 0, len:Number = self.m_deferreds.length; i < len; i++) {
                handle(self, self.m_deferreds[i]);
            }
            self.m_deferreds = null;
        }

        /**
         * Takes a potentially misbehaving resolver function and make sure
         * `onFulfilled` and `onRejected` are only called once.
         *
         * Makes no guarantees about asynchrony.
         */
        private static function doResolve(fn:Function, self:Promise.<T>):void {
            var done:Boolean = false;
            try {
                fn(
                    function(value:*):* {
                        if (done) return;
                        done = true;
                        Promise.<T>.privateResolve(self, value);
                    },
                    function(reason:*):* {
                        if (done) return;
                        done = true;
                        Promise.<T>.privateReject(self, reason);
                    }
                );
            }
            catch (ex:*) {
                if (done) return;
                done = true;
                Promise.<T>.privateReject(self, ex);
            }
        }

        /**
         * The `Promise.allSettled()` static method takes an iterable of promises
         * as input and returns a single Promise.
         * This returned promise fulfills when all of the input's promises settle
         * (including when an empty iterable is passed), with an array of objects that
         * describe the outcome of each promise.
         *
         * # Example
         *
         * ```
         * const promise1 = Promise.<Number>.resolve(3);
         * const promise2 = new Promise.<Number>((resolve, reject) => {
         *     setTimeout(reject, 100, "foo");
         * });
         * Promise.<Number>.allSettled([promise1, promise2])
         *     .then(results => {
         *         for each (const result in results) {
         *             trace(result.status);
         *         }
         *     });
         * // Expected output:
         * // "fulfilled"
         * // "rejected"
         * ```
         * 
         * @return A `Promise` that is:
         *
         * - **Already fulfilled,** if the iterable passed is empty.
         * - **Asynchronously fulfilled,** when all promises in the given
         *   iterable have settled (either fulfilled or rejected).
         *   The fulfillment value is an array of objects, each describing the
         *   outcome of one promise in the iterable, in the order of the promises passed,
         *   regardless of completion order. Each outcome object has the following properties:
         *     - `status`: A string, either `"fulfilled"` or `"rejected"`, indicating the eventual state of the promise.
         *     - `value`: Only present if `status` is `"fulfilled"`. The value that the promise was fulfilled with.
         *     - `reason`: Only present if `status` is `"rejected"`. The reason that the promise was rejected with.
         *
         * If the iterable passed is non-empty but contains no pending promises,
         * the returned promise is still asynchronously (instead of synchronously) fulfilled.
         */
        public static function allSettled(promises: Array): Promise.<T> {
            return new Promise(function(resolve:Function, reject:Function):void {
                var args:Array = promises.slice(0);
                if (args.length === 0) {
                    resolve([]);
                    return;
                }
                var remaining:Number = args.length;
                function res(i:Number, val:*):void {
                    if (val is Promise) {
                        Promise(val).then(
                            function(val:*):* {
                                res(i, val);
                            },
                            function(e:*):* {
                                args[i] = { status: 'rejected', reason: e };
                                if (--remaining === 0) {
                                    resolve(args);
                                }
                            }
                        );
                        return;
                    }
                    args[i] = { status: 'fulfilled', value: val };
                    if (--remaining === 0) {
                        resolve(args);
                    }
                }
                for (var i:Number = 0; i < args.length; ++i) {
                    res(i, args[i]);
                }
            });
        } // allSettled

        /**
         * [developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/any)
         */
        public static function any(promises:Array):Promise.<T> {
            return new Promise(function(resolve:Function, reject:Function):void {
                var args:Array = promises.slice(0);
                if (args.length === 0) {
                    reject(undefined);
                    return;
                }
                var rejectionReasons:Array = [];
                for (var i:Number = 0; i < args.length; ++i) {
                    try {
                        Promise.<T>.resolve(args[i])
                            .then(resolve)
                            .catch(function(error:*):* {
                                rejectionReasons.push(error);
                                if (rejectionReasons.length === args.length) {
                                    reject(
                                        new AggregateError(
                                            rejectionReasons,
                                            'All promises were rejected'
                                        )
                                    );
                                }
                            });
                    }
                    catch (ex:*) {
                        reject(ex);
                    }
                }
            });
        } // any

        public function finally(callback:Function):Promise.<T> {
            return this.then(
                function(value:*):* {
                    return Promise.<T>.resolve(callback()).then(function(_:*):* {
                        return value;
                    });
                },
                function(reason:*):* {
                    return Promise.<T>.resolve(callback()).then(function(_:*):* {
                        return Promise.<T>.reject(reason);
                    });
                }
            );
        }

        public function catch(onRejected: Function):Promise.<T> {
            return this.then(null, onRejected);
        }

        public function then.<U, E>(onFulfilled: (data: T) => U, onRejected: (error: *) => E = null):Promise.<T> {
            var prom = new Promise.<T>(function(_a:*, _b:*):void {});
            Promise.<T>.handle(this, new PromiseHandler(onFulfilled, onRejected, prom));
            return prom;
        }

        /**
         * [developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/all)
         */
        public static function all(promises:Array):Promise.<T> {
            return new Promise(function(resolve:Function, reject:Function):void {
                var args:Array = promises.slice(0);
                if (args.length === 0) {
                    resolve([]);
                    return;
                }
                var remaining:Number = args.length;

                function res(i:Number, val:*):void {
                    try {
                        if (val is Promise) {
                            Promise(val).then(
                                val => {
                                    res(i, val);
                                },
                                reject
                            );
                            return;
                        }
                        args[i] = val;
                        if (--remaining === 0) {
                            resolve(args);
                        }
                    }
                    catch (ex:*) {
                        reject(ex);
                    }
                }

                for (var i:Number = 0; i < args.length; i++) {
                    res(i, args[i]);
                }
            });
        } // all

        /**
         * [developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/resolve)
         */
        public static function resolve(value: *):Promise.<T> {
            if (value is Promise) {
                return Promise.<T>(value);
            }

            return new Promise.<T>((resolve, reject) => {
                resolve(value);
            });
        }

        /**
         * [developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/reject)
         */
        public static function reject(value: *): Promise.<T> {
            return new Promise((resolve, reject) => {
                reject(value);
            });
        }

        /**
         * [developer.mozilla.org](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Promise/race)
         */
        public static function race(promises:Array):Promise.<T> {
            return new Promise.<T>(function(resolve:Function, reject:Function):void {
                for each (var arg:* in promises) {
                    Promise.<T>.resolve(arg).then(resolve, reject);
                }
            });
        }

        private static function _immediateFn(fn:Function):void {
            setTimeout(fn, 0);
        }

        private static function _unhandledRejectionFn(err:*):void {
            trace("Possible unhandled Promise rejection:", err);
        }
    }
}