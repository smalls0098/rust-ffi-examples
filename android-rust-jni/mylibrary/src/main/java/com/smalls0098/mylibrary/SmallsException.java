package com.smalls0098.mylibrary;

import androidx.annotation.Keep;

@Keep
public class SmallsException extends Exception{
    public SmallsException() {
        super();
    }
    public SmallsException(String message) {
        super(message);
    }
    public SmallsException(String message, Throwable cause) {
        super(message, cause);
    }
    public SmallsException(Throwable cause) {
        super(cause);
    }
    protected SmallsException(String message, Throwable cause, boolean enableSuppression, boolean writableStackTrace) {
        super(message, cause, enableSuppression, writableStackTrace);
    }
}
