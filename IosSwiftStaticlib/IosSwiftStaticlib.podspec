Pod::Spec.new do |s|
  s.name             = 'IosSwiftStaticlib'
  s.version          = '0.1.0'
  s.summary          = 'A short description of ios-swift-staticlib.'
  s.description      = <<-DESC
TODO: Add long description of the pod here.
                       DESC

  s.homepage         = 'https://github.com/smalls0098/rust-ffi'
  s.license          = { :type => 'Apache-2.0', :file => 'LICENSE' }
  s.author           = { 'smalls' => 'smalls0098@gmail.com' }
  s.source           = { :git => 'https://github.com/smalls0098/rust-ffi.git', :tag => s.version.to_s }

  s.ios.deployment_target = '9.0'

  s.source_files = 'IosSwiftStaticlib/Classes/**/*'
  s.vendored_libraries = 'IosSwiftStaticlib/Classes/librust_ios.a'
  s.public_header_files = 'IosSwiftStaticlib/Classes/rust-ios.h'
  
  s.swift_version = ['5.0']
  
  s.pod_target_xcconfig = {
    #这个没设置会编译不过去
    'ENABLE_BITCODE' => 'NO',
    'VALID_ARCHS' => 'armv7 armv7s x86_64 i386',
    #生产环境设置的值,可以跳过模拟器验证
#    'VALID_ARCHS' => 'arm64 armv7 armv7s',
#    'VALID_ARCHS[sdk=iphonesimulator*]' => '',
  }
end
