// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

//! Futures provides the futures generated by [`Operator`]
//!
//! By using futures, users can add more options for operation.

use std::collections::HashMap;
use std::future::IntoFuture;
use std::ops::RangeBounds;
use std::time::Duration;

use futures::Future;

use crate::raw::*;
use crate::*;

/// OperatorFuture is the future generated by [`Operator`].
///
/// The future will consume all the input to generate a future.
///
/// # NOTES
///
/// This struct is by design to keep in crate. We don't want
/// users to use this struct directly.
pub struct OperatorFuture<I, O, F: Future<Output = Result<O>>> {
    /// The accessor to the underlying object storage
    acc: Accessor,
    /// The path of string
    path: String,
    /// The input args
    args: I,
    /// The function which will move all the args and return a static future
    f: fn(Accessor, String, I) -> F,
}

impl<I, O, F: Future<Output = Result<O>>> OperatorFuture<I, O, F> {
    /// # NOTES
    ///
    /// This struct is by design to keep in crate. We don't want
    /// users to use this struct directly.
    pub(crate) fn new(
        inner: Accessor,
        path: String,
        args: I,
        f: fn(Accessor, String, I) -> F,
    ) -> Self {
        OperatorFuture {
            acc: inner,
            path,
            args,
            f,
        }
    }
}

impl<I, O, F: Future<Output = Result<O>>> OperatorFuture<I, O, F> {
    /// Change the operation's args.
    fn map(mut self, f: impl FnOnce(I) -> I) -> Self {
        self.args = f(self.args);
        self
    }
}

impl<I, O, F> IntoFuture for OperatorFuture<I, O, F>
where
    F: Future<Output = Result<O>>,
{
    type Output = Result<O>;
    type IntoFuture = F;

    fn into_future(self) -> Self::IntoFuture {
        (self.f)(self.acc, self.path, self.args)
    }
}

/// Future that generated by [`Operator::stat_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FutureStat<F> = OperatorFuture<OpStat, Metadata, F>;

impl<F: Future<Output = Result<Metadata>>> FutureStat<F> {
    /// Set the If-Match for this operation.
    pub fn if_match(self, v: &str) -> Self {
        self.map(|args| args.with_if_match(v))
    }

    /// Set the If-None-Match for this operation.
    pub fn if_none_match(self, v: &str) -> Self {
        self.map(|args| args.with_if_none_match(v))
    }

    /// Set the version for this operation.
    pub fn version(self, v: &str) -> Self {
        self.map(|args| args.with_version(v))
    }
}

/// Future that generated by [`Operator::presign_stat_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FuturePresignStat<F> = OperatorFuture<(OpStat, Duration), PresignedRequest, F>;

impl<F: Future<Output = Result<PresignedRequest>>> FuturePresignStat<F> {
    /// Sets the content-disposition header that should be sent back by the remote read operation.
    pub fn override_content_disposition(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_override_content_disposition(v), dur))
    }

    /// Sets the cache-control header that should be sent back by the remote read operation.
    pub fn override_cache_control(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_override_cache_control(v), dur))
    }

    /// Sets the content-type header that should be sent back by the remote read operation.
    pub fn override_content_type(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_override_content_type(v), dur))
    }

    /// Set the If-Match of the option
    pub fn if_match(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_if_match(v), dur))
    }

    /// Set the If-None-Match of the option
    pub fn if_none_match(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_if_none_match(v), dur))
    }
}

/// Future that generated by [`Operator::presign_read_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FuturePresignRead<F> = OperatorFuture<(OpRead, Duration), PresignedRequest, F>;

impl<F: Future<Output = Result<PresignedRequest>>> FuturePresignRead<F> {
    /// Sets the content-disposition header that should be sent back by the remote read operation.
    pub fn override_content_disposition(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_override_content_disposition(v), dur))
    }

    /// Sets the cache-control header that should be sent back by the remote read operation.
    pub fn override_cache_control(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_override_cache_control(v), dur))
    }

    /// Sets the content-type header that should be sent back by the remote read operation.
    pub fn override_content_type(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_override_content_type(v), dur))
    }

    /// Set the If-Match of the option
    pub fn if_match(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_if_match(v), dur))
    }

    /// Set the If-None-Match of the option
    pub fn if_none_match(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_if_none_match(v), dur))
    }
}

/// Future that generated by [`Operator::presign_write_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FuturePresignWrite<F> = OperatorFuture<(OpWrite, Duration), PresignedRequest, F>;

impl<F: Future<Output = Result<PresignedRequest>>> FuturePresignWrite<F> {
    /// Set the content type of option
    pub fn content_type(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_content_type(v), dur))
    }

    /// Set the content disposition of option
    pub fn content_disposition(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_content_disposition(v), dur))
    }

    /// Set the content type of option
    pub fn cache_control(self, v: &str) -> Self {
        self.map(|(args, dur)| (args.with_cache_control(v), dur))
    }
}

/// Future that generated by [`Operator::read_with`] or [`Operator::reader_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FutureRead<F> = OperatorFuture<(OpRead, OpReader), Buffer, F>;

impl<F: Future<Output = Result<Buffer>>> FutureRead<F> {
    /// Set the If-Match for this operation.
    pub fn if_match(self, v: &str) -> Self {
        self.map(|(args, op_reader)| (args.with_if_match(v), op_reader))
    }

    /// Set the If-None-Match for this operation.
    pub fn if_none_match(self, v: &str) -> Self {
        self.map(|(args, op_reader)| (args.with_if_none_match(v), op_reader))
    }

    /// Set the version for this operation.
    pub fn version(self, v: &str) -> Self {
        self.map(|(args, op_reader)| (args.with_version(v), op_reader))
    }

    /// Set the executor for this operation.
    pub fn executor(self, executor: Executor) -> Self {
        self.map(|(args, op_reader)| (args.with_executor(executor), op_reader))
    }

    /// Set the range header for this operation.
    pub fn range(self, range: impl RangeBounds<u64>) -> Self {
        self.map(|(args, op_reader)| (args.with_range(range.into()), op_reader))
    }

    /// Set the concurrent read task amount.
    pub fn concurrent(self, concurrent: usize) -> Self {
        self.map(|(args, op_reader)| (args, op_reader.with_concurrent(concurrent)))
    }

    /// Set the chunk size for this operation.
    pub fn chunk(self, chunk_size: usize) -> Self {
        self.map(|(args, op_reader)| (args, op_reader.with_chunk(chunk_size)))
    }
}

/// Future that generated by [`Operator::read_with`] or [`Operator::reader_with`].
///
/// Users can add more options by public functions provided by this struct.
///
/// # Notes
///
/// `(OpRead, ())` is a trick to make sure `FutureReader` is different from `FutureRead`
pub type FutureReader<F> = OperatorFuture<(OpRead, OpReader), Reader, F>;

impl<F: Future<Output = Result<Reader>>> FutureReader<F> {
    /// Set the version for this operation.
    pub fn version(self, v: &str) -> Self {
        self.map(|(op_read, op_reader)| (op_read.with_version(v), op_reader))
    }

    /// Set the concurrent read task amount.
    pub fn concurrent(self, concurrent: usize) -> Self {
        self.map(|(op_read, op_reader)| (op_read, op_reader.with_concurrent(concurrent)))
    }

    /// Set the chunk size for this reader.
    pub fn chunk(self, chunk_size: usize) -> Self {
        self.map(|(op_read, op_reader)| (op_read, op_reader.with_chunk(chunk_size)))
    }

    /// Set the gap size for this reader.
    pub fn gap(self, gap_size: usize) -> Self {
        self.map(|(op_read, op_reader)| (op_read, op_reader.with_gap(gap_size)))
    }
}

/// Future that generated by [`Operator::write_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FutureWrite<F> = OperatorFuture<(OpWrite, OpWriter, Buffer), (), F>;

impl<F: Future<Output = Result<()>>> FutureWrite<F> {
    /// Set the append mode of op.
    ///
    /// If the append mode is set, the data will be appended to the end of the file.
    ///
    /// # Notes
    ///
    /// Service could return `Unsupported` if the underlying storage does not support append.
    pub fn append(self, v: bool) -> Self {
        self.map(|(args, options, bs)| (args.with_append(v), options, bs))
    }

    /// Set the buffer size of op.
    ///
    /// If buffer size is set, the data will be buffered by the underlying writer.
    ///
    /// ## NOTE
    ///
    /// Service could have their own minimum buffer size while perform write operations like
    /// multipart uploads. So the buffer size may be larger than the given buffer size.
    pub fn chunk(self, v: usize) -> Self {
        self.map(|(args, options, bs)| (args, options.with_chunk(v), bs))
    }

    /// Set the maximum concurrent write task amount.
    pub fn concurrent(self, v: usize) -> Self {
        self.map(|(args, options, bs)| (args.with_concurrent(v), options, bs))
    }

    /// Set the content type of option
    pub fn cache_control(self, v: &str) -> Self {
        self.map(|(args, options, bs)| (args.with_cache_control(v), options, bs))
    }

    /// Set the content type of option
    pub fn content_type(self, v: &str) -> Self {
        self.map(|(args, options, bs)| (args.with_content_type(v), options, bs))
    }

    /// Set the content disposition of option
    pub fn content_disposition(self, v: &str) -> Self {
        self.map(|(args, options, bs)| (args.with_content_disposition(v), options, bs))
    }

    /// Set the executor for this operation.
    pub fn executor(self, executor: Executor) -> Self {
        self.map(|(args, options, bs)| (args.with_executor(executor), options, bs))
    }

    /// Set the If-None-Match for this operation.
    pub fn if_none_match(self, s: &str) -> Self {
        self.map(|(args, options, bs)| (args.with_if_none_match(s), options, bs))
    }

    /// Set the If-Not-Exist for this operation.
    pub fn if_not_exists(self, b: bool) -> Self {
        self.map(|(args, options, bs)| (args.with_if_not_exists(b), options, bs))
    }

    /// Set the user defined metadata of the op
    ///
    /// ## Notes
    ///
    /// we don't need to include the user defined metadata prefix in the key
    /// every service will handle it internally
    pub fn user_metadata(self, data: impl IntoIterator<Item = (String, String)>) -> Self {
        self.map(|(args, options, bs)| {
            (
                args.with_user_metadata(HashMap::from_iter(data)),
                options,
                bs,
            )
        })
    }
}

/// Future that generated by [`Operator::writer_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FutureWriter<F> = OperatorFuture<(OpWrite, OpWriter), Writer, F>;

impl<F: Future<Output = Result<Writer>>> FutureWriter<F> {
    /// Set the append mode of op.
    ///
    /// If the append mode is set, the data will be appended to the end of the file.
    ///
    /// ## Notes
    ///
    /// Service could return `Unsupported` if the underlying storage does not support append.
    pub fn append(self, v: bool) -> Self {
        self.map(|(args, options)| (args.with_append(v), options))
    }

    /// Set the chunk size of op.
    ///
    /// If chunk size is set, the data will be chunked by the underlying writer.
    ///
    /// ## NOTE
    ///
    /// Service could have their own limitation for chunk size. It's possible that chunk size
    /// is not equal to the given chunk size.
    ///
    /// For example:
    ///
    /// - AWS S3 requires the part size to be in [5MiB, 5GiB].
    /// - GCS requires the part size to be aligned with 256 KiB.
    ///
    /// The services will alter the chunk size to meet their requirements.
    pub fn chunk(self, v: usize) -> Self {
        self.map(|(args, options)| (args, options.with_chunk(v)))
    }

    /// Set the maximum concurrent write task amount.
    pub fn concurrent(self, v: usize) -> Self {
        self.map(|(args, options)| (args.with_concurrent(v), options))
    }

    /// Set the content type of option
    pub fn cache_control(self, v: &str) -> Self {
        self.map(|(args, options)| (args.with_cache_control(v), options))
    }

    /// Set the content type of option
    pub fn content_type(self, v: &str) -> Self {
        self.map(|(args, options)| (args.with_content_type(v), options))
    }

    /// Set the content disposition of option
    pub fn content_disposition(self, v: &str) -> Self {
        self.map(|(args, options)| (args.with_content_disposition(v), options))
    }

    /// Set the executor for this operation.
    pub fn executor(self, executor: Executor) -> Self {
        self.map(|(args, options)| (args.with_executor(executor), options))
    }

    /// Set the user defined metadata of the op
    ///
    /// ## Notes
    ///
    /// we don't need to include the user defined metadata prefix in the key.
    /// every service will handle it internally
    pub fn user_metadata(self, data: impl IntoIterator<Item = (String, String)>) -> Self {
        self.map(|(args, options)| (args.with_user_metadata(HashMap::from_iter(data)), options))
    }
}

/// Future that generated by [`Operator::delete_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FutureDelete<F> = OperatorFuture<OpDelete, (), F>;

impl<F: Future<Output = Result<()>>> FutureDelete<F> {
    /// Change the version of this delete operation.
    pub fn version(self, v: &str) -> Self {
        self.map(|args| args.with_version(v))
    }
}

/// Future that generated by [`Operator::list_with`] or [`Operator::lister_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FutureList<F> = OperatorFuture<OpList, Vec<Entry>, F>;

impl<F: Future<Output = Result<Vec<Entry>>>> FutureList<F> {
    /// The limit passed to underlying service to specify the max results
    /// that could return per-request.
    ///
    /// Users could use this to control the memory usage of list operation.
    pub fn limit(self, v: usize) -> Self {
        self.map(|args| args.with_limit(v))
    }

    /// The start_after passes to underlying service to specify the specified key
    /// to start listing from.
    pub fn start_after(self, v: &str) -> Self {
        self.map(|args| args.with_start_after(v))
    }

    /// The recursive is used to control whether the list operation is recursive.
    ///
    /// - If `false`, list operation will only list the entries under the given path.
    /// - If `true`, list operation will list all entries that starts with given path.
    ///
    /// Default to `false`.
    pub fn recursive(self, v: bool) -> Self {
        self.map(|args| args.with_recursive(v))
    }

    /// The version is used to control whether the object versions should be returned.
    ///
    /// - If `false`, list operation will not return with object versions
    /// - If `true`, list operation will return with object versions if object versioning is supported
    ///   by the underlying service
    ///
    /// Default to `false`
    pub fn version(self, v: bool) -> Self {
        self.map(|args| args.with_version(v))
    }
}

/// Future that generated by [`Operator::list_with`] or [`Operator::lister_with`].
///
/// Users can add more options by public functions provided by this struct.
pub type FutureLister<F> = OperatorFuture<OpList, Lister, F>;

impl<F: Future<Output = Result<Lister>>> FutureLister<F> {
    /// The limit passed to underlying service to specify the max results
    /// that could return per-request.
    ///
    /// Users could use this to control the memory usage of list operation.
    pub fn limit(self, v: usize) -> Self {
        self.map(|args| args.with_limit(v))
    }

    /// The start_after passes to underlying service to specify the specified key
    /// to start listing from.
    pub fn start_after(self, v: &str) -> Self {
        self.map(|args| args.with_start_after(v))
    }

    /// The recursive is used to control whether the list operation is recursive.
    ///
    /// - If `false`, list operation will only list the entries under the given path.
    /// - If `true`, list operation will list all entries that starts with given path.
    ///
    /// Default to `false`.
    pub fn recursive(self, v: bool) -> Self {
        self.map(|args| args.with_recursive(v))
    }

    /// The version is used to control whether the object versions should be returned.
    ///
    /// - If `false`, list operation will not return with object versions
    /// - If `true`, list operation will return with object versions if object versioning is supported
    ///   by the underlying service
    ///
    /// Default to `false`
    pub fn version(self, v: bool) -> Self {
        self.map(|args| args.with_version(v))
    }
}
