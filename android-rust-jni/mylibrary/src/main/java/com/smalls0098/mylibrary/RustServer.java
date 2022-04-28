package com.smalls0098.mylibrary;

import androidx.annotation.Keep;

@Keep
public class RustServer {

    static {
        System.loadLibrary("rust");
    }

    public static native String getVersion();

}
