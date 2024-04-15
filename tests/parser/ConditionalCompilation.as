package com.nintendo.metroid {
    /**
     * Comment 1 (overriden by the "Communication facility" comment).
     */
    CONFIG::DEBUG
    /**
     * Communication facility.
     */
    [Adherence(type = "efficient")]
    /**
     * Dispatched when a message is received.
     */
    [Event(name = "received", type = "com.nintendo.metroid.MessageEvent")]
    public class CommunicationCenter extends EventDispatcher {
        CONFIG::DEBUG {
            protected var x: T1, y: T2, z: T3;
            protected function f1(): void {}
        }
        CONFIG::RELEASE protected var w: T4;
    }
}