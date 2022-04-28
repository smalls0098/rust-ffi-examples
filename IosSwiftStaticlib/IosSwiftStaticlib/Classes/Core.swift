//
//  Core.swift
//  ios-swift-staticlib
//
//  Created by Ming on 2022/4/27.
//

import Foundation

public struct TokenInfoCore {
    public var token: String
    public var core: String
}

public class Core {
    
    /// 单例模式
    public static let shared = Core()
    
    /// 获取版本号
    public func getVersion() -> String {
        let version = get_version()
        let swiftVersion = String(cString: version!)
        rust_free(UnsafeMutablePointer(mutating: version))
        return swiftVersion
    }
    
    /// 获取token
    public func getToken() -> TokenInfoCore {
        let info = get_token("test".cString(using: .utf8))
        let ret = TokenInfoCore(
            token: String(cString: info.token!),
            core: String(cString: info.core!)
        )
        return ret
    }
    
}
