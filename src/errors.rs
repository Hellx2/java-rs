use std::fmt::Display;

// TODO: Alter so that it says {stringify($name)}: {...error args}

macro_rules! define_error {
    ($name: ident) => {
        #[derive(Debug, Clone, PartialEq)]
        pub struct $name {
            message: String,
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.message)
            }
        }

        impl $name {
            pub fn new(message: String) -> Self {
                Self { message }
            }
        }
    };
    ($name: ident, $($arg: ident # $type: ty),+) => {
        pub struct $name {
            message: String,
            $($arg: $type),+
        }

        impl Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}{}", self.message, $(self.$arg),+)
            }
        }

        impl $name {
            pub fn new(message: String, $($arg: $type),+) -> Self {
                Self { message, $($arg),+ }
            }
        }
    };
}

macro_rules! define_std_error {
    ($name: ident) => {
        #[derive(Debug)]
        pub struct $name {
            message: String,
            cause: Option<Box<dyn std::error::Error>>,
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if let Some(err) = &self.cause {
                    write!(f, "{}: {}", self.message, err)
                } else {
                    write!(f, "{}", self.message)
                }
            }
        }

        impl $name {
            pub fn new(message: String, cause: Option<Box<dyn std::error::Error>>) -> Self {
                Self { message, cause }
            }
        }
    };
}

macro_rules! define_error_no_subsequent {
    ($name: ident) => {
        #[derive(Debug)]
        pub struct $name {
            message: Option<String>,
            cause: Option<Box<dyn std::error::Error>>,
        }

        impl std::fmt::Display for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if let Some(err) = &self.cause {
                    write!(f, "{}", err)
                } else {
                    write!(f, "{}", self.message.as_ref().unwrap())
                }
            }
        }

        impl $name {
            pub fn new(message: String) -> Self {
                Self {
                    message: Some(message),
                    cause: None,
                }
            }

            pub fn new_cause(cause: Box<dyn std::error::Error>) -> Self {
                Self {
                    message: None,
                    cause: Some(cause),
                }
            }
        }
    };
}

define_error!(AbstractMethodError);
define_error!(IncompatibleClassChangeException);
define_error!(ArithmeticException);
define_error!(ArrayIndexOutOfBoundsException, index # i32);
define_error!(ArrayStoreException);
define_error!(AssertionError);
define_error!(ClassCastException);
define_error!(ClassCircularityError);
define_error!(ClassFormatError);
define_error!(CloneNotSupportedError);
define_error!(EnumConstantNotPresentException);
define_error!(IllegalAccessError);
define_error!(IllegalAccessException);
define_error!(IllegalMonitorStateException);
define_error!(IllegalThreadStateException);
define_error!(IncompatibleClassChangeError);
define_error!(IndexOutOfBoundsException, index # i32);
define_error!(InstantiationError);
define_error!(InstantiationException);
define_error!(InterruptedException);
define_error!(NegativeArraySizeException);
define_error!(NoClassDefFoundError);
define_error!(NoSuchFieldError);
define_error!(NoSuchFieldException);
define_error!(NoSuchMethodError);
define_error!(NoSuchMethodException);
define_error!(NullPointerException);
define_error!(NumberFormatException);
define_error!(OutOfMemoryError);
define_error!(StackOverflowError);
define_error!(StringIndexOutOfBoundsException, index # i32);
define_error!(UnknownError);
define_error!(UnsatisfiedLinkError);
define_error!(UnsupportedClassVersionError);
define_error!(VerifyError);
define_error!(GenericSignatureFormatError);
define_error!(InaccessibleObjectException);
define_error!(MalformedParameterizedTypeException);
define_error!(MalformedParametersException);
define_error!(InvalidModuleDescriptorException);
define_error!(StringConcatException);

// TODO: Alter these to correctly match JVM
define_error!(AnnotationTypeMismatchException);
define_error!(IncompleteAnnotationException);

define_error_no_subsequent!(ExceptionInInitializerError);

define_std_error!(BootstrapMethodError);
define_std_error!(ClassNotFoundException);
define_std_error!(IllegalArgumentException);
define_std_error!(IllegalCallerException);
define_std_error!(IllegalStateException);
define_std_error!(InternalError);
define_std_error!(LayerInstantiationException);
define_std_error!(LinkageError);
define_std_error!(MatchException);
define_std_error!(ReflectiveOperationException);
define_std_error!(RuntimeException);
define_std_error!(SecurityException);
define_std_error!(TypeNotPresentException);
define_std_error!(UnsupportedOperationException);
define_std_error!(VirtualMachineError);
define_std_error!(WrongThreadException);
define_std_error!(UndeclaredThrowableException);
define_std_error!(ResolutionException);
define_std_error!(LambdaConversionException);
define_std_error!(WrongMethodTypeException);
define_std_error!(AnnotationFormatError);

pub mod io {
    use super::*;

    define_error!(CharConversionException);
    define_error!(EOFException);
    define_error!(FileNotFoundException);
    define_error!(InterruptedIOException);
    define_error!(NotActiveException);
    define_error!(NotSerializableException);
    define_error!(StreamCorruptedException);
    define_error!(SyncFailedException);
    define_error!(UTFDataFormatException);
    define_error!(UnsupportedEncodingException);

    define_std_error!(IOException);
    define_std_error!(InvalidObjectException);
    define_std_error!(ObjectStreamException);
    define_std_error!(UncheckedIOException);
    define_std_error!(WriteAbortedException);

    // TODO: OptionalDataException

    #[derive(Debug)]
    pub struct InvalidClassException {
        cname: String,
        message: String,
        cause: Option<Box<dyn std::error::Error>>,
    }

    impl std::fmt::Display for InvalidClassException {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(err) = &self.cause {
                write!(f, "{};{}: {}", self.cname, self.message, err)
            } else {
                write!(f, "{};{}", self.cname, self.message)
            }
        }
    }

    impl InvalidClassException {
        pub fn new(
            cname: String,
            message: String,
            cause: Option<Box<dyn std::error::Error>>,
        ) -> Self {
            Self {
                cname,
                message,
                cause,
            }
        }
    }

    #[derive(Debug)]
    pub struct IOError {
        cause: Option<Box<dyn std::error::Error>>,
    }

    impl std::fmt::Display for IOError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            if let Some(err) = &self.cause {
                write!(f, "{err}")
            } else {
                write!(f, "")
            }
        }
    }

    impl IOError {
        pub fn new(cause: Option<Box<dyn std::error::Error>>) -> Self {
            Self { cause }
        }
    }
}

pub mod net {
    use super::*;

    define_error!(BindException);
    define_error!(ConnectException);
    define_error!(MalformedURLException);
    define_error!(NoRouteToHostException);
    define_error!(PortUnreachableException);
    define_error!(ProtocolException);
    define_error!(SocketTimeoutException);
    define_error!(UnknownHostException);
    define_error!(UnknownServiceException);

    define_std_error!(SocketException);

    // TODO: HttpRetryException
    // TODO: URISyntaxException
}

pub mod nio {
    pub mod file {
        use super::super::*;

        define_error!(AccessDeniedException);
        define_error!(DirectoryNotEmptyException);
        define_error!(FileAlreadyExistsException);
        define_error!(FileSystemAlreadyExistsException);
        define_error!(FileSystemLoopException);
        define_error!(FileSystemNotFoundException);
        define_error!(NotDirectoryException);
        define_error!(ProviderMismatchException);
        define_error!(ProviderNotFoundException);

        macro_rules! file_system_exception {
            ($name: ident) => {
                #[derive(Debug, Clone, PartialEq)]
                pub struct $name {
                    file: String,
                    other: Option<String>,
                    reason: Option<String>,
                }
                impl Display for $name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(
                            f,
                            "{} {} {}",
                            self.file,
                            self.other.clone().unwrap_or("".to_string()),
                            self.reason.clone().unwrap_or("".to_string())
                        )
                    }
                }
                impl $name {
                    pub fn new(
                        file: String,
                        other: Option<String>,
                        reason: Option<String>,
                    ) -> Self {
                        Self {
                            file,
                            other,
                            reason,
                        }
                    }
                }
            };
        }

        pub struct InvalidPathException {
            input: String,
            reason: String,
            index: i32,
        }
        impl Display for InvalidPathException {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{} {} {}", self.input, self.reason, self.index)
            }
        }

        impl InvalidPathException {
            pub fn new(input: String, reason: String, index: i32) -> Self {
                Self {
                    input,
                    reason,
                    index,
                }
            }
        }

        file_system_exception!(FileSystemException);
        file_system_exception!(NoSuchFileException);
        file_system_exception!(NotLinkException);

        // CoderMalfunctionError

        #[derive(Debug)]
        pub struct DirectoryIteratorException {
            cause: Option<Box<dyn std::error::Error>>,
        }

        impl Display for DirectoryIteratorException {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if let Some(err) = &self.cause {
                    write!(f, "{}", err)
                } else {
                    write!(f, "")
                }
            }
        }

        impl DirectoryIteratorException {
            pub fn new(cause: Option<Box<dyn std::error::Error>>) -> Self {
                Self { cause }
            }
        }

        #[derive(Debug)]
        pub struct CoderMalfunctionError {
            cause: Option<Box<dyn std::error::Error>>,
        }

        impl Display for CoderMalfunctionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                if let Some(err) = &self.cause {
                    write!(f, "{}", err)
                } else {
                    write!(f, "")
                }
            }
        }

        impl CoderMalfunctionError {
            pub fn new(cause: Option<Box<dyn std::error::Error>>) -> Self {
                Self { cause }
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct MalformedInputException {
            input_length: i32,
        }

        impl Display for MalformedInputException {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}: Input length = {}",
                    stringify!(MalformedInputException),
                    self.input_length
                )
            }
        }
        impl MalformedInputException {
            pub fn new(input_length: i32) -> Self {
                Self { input_length }
            }
        }
        impl Default for MalformedInputException {
            fn default() -> Self {
                Self::new(0)
            }
        }

        #[derive(Debug, Clone, PartialEq)]
        pub struct UnmappableCharacterException {
            input_length: i32,
        }

        impl Display for UnmappableCharacterException {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(
                    f,
                    "{}: Input length = {}",
                    stringify!(MalformedInputException),
                    self.input_length
                )
            }
        }
        impl UnmappableCharacterException {
            pub fn new(input_length: i32) -> Self {
                Self { input_length }
            }
        }
        impl Default for UnmappableCharacterException {
            fn default() -> Self {
                Self::new(0)
            }
        }

        // ReadOnlyFileSystemException
        macro_rules! define_no_args_error {
            ($name: ident) => {
                #[derive(Debug, Clone, PartialEq)]
                pub struct $name;
                impl Display for $name {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(f, stringify!($name))
                    }
                }
                impl $name {
                    pub fn new() -> Self {
                        Self {}
                    }
                }

                impl Default for $name {
                    fn default() -> Self {
                        Self::new()
                    }
                }
            };
        }

        define_no_args_error!(ClosedFileSystemException);
        define_no_args_error!(ClosedDirectoryStreamException);
        define_no_args_error!(ClosedWatchServiceException);
        define_no_args_error!(ReadOnlyFileSystemException);

        // TODO: AtomicMoveNotSupportedException
    }
}

pub mod security {
    // TODO: Permission, errors here require Permission
}

pub mod time {
    //use super::*;

    define_std_error!(DateTimeException);
    define_std_error!(ZoneRulesException);
    define_std_error!(UnsupportedTemporalTypeException);
    // TODO: DateTimeParseException
}

// TODO: java.util

define_error!(ParseException, error_offset # i32);

#[cfg(test)]
mod tests {
    use crate::errors::AbstractMethodError;

    #[test]
    fn error_check() {
        assert_eq!(
            AbstractMethodError::new("e".to_string()),
            AbstractMethodError {
                message: "e".to_string()
            }
        )
    }
}
