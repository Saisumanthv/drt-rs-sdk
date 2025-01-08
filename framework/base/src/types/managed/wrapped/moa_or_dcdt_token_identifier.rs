use alloc::string::ToString;

use crate::{
    abi::{TypeAbi, TypeAbiFrom, TypeName},
    api::{HandleConstraints, ManagedTypeApi},
    codec::*,
    derive::ManagedVecItem,
    formatter::{FormatByteReceiver, SCDisplay, SCLowerHex},
    proxy_imports::TestTokenIdentifier,
    types::{ManagedBuffer, ManagedOption, ManagedRef, ManagedType, TokenIdentifier},
};

use crate as dharitri_sc; // required by the ManagedVecItem derive

/// Specialized type for handling either MOA or DCDT token identifiers.
///
/// Equivalent to a structure of the form
/// ```
/// # use dharitri_sc::{api::ManagedTypeApi, types::TokenIdentifier};
/// enum MoaOrDcdtTokenIdentifier<M: ManagedTypeApi> {
///     Moa,
///     Dcdt(TokenIdentifier<M>),
/// }
/// ```
///
/// It is, however more optimized than that. Its implementation is based on `ManagedOption`.
///
/// MOA a special, invalid token identifier handle. This way we can fit it inside a single i32 in memory.
#[repr(transparent)]
#[derive(ManagedVecItem, Clone)]
pub struct MoaOrDcdtTokenIdentifier<M: ManagedTypeApi> {
    pub(crate) data: ManagedOption<M, TokenIdentifier<M>>,
}

impl<M: ManagedTypeApi> MoaOrDcdtTokenIdentifier<M> {
    /// This special representation is interpreted as the MOA token.
    #[allow(clippy::needless_borrow)] // clippy is wrog here, there is no other way
    pub const MOA_REPRESENTATION: &'static [u8; 3] = &b"MOA";

    /// New instance of the special MOA token representation.
    #[inline]
    pub fn moa() -> Self {
        Self {
            data: ManagedOption::none(),
        }
    }

    /// DCDT instance, containing an DCDT token identifier.
    #[inline]
    pub fn dcdt<TI>(token_identifier: TI) -> Self
    where
        TokenIdentifier<M>: From<TI>,
    {
        Self {
            data: ManagedOption::some(TokenIdentifier::from(token_identifier)),
        }
    }

    pub fn from_opt_raw_handle(opt_handle: Option<M::ManagedBufferHandle>) -> Self {
        match opt_handle {
            Some(handle) => Self::dcdt(TokenIdentifier::from_handle(handle)),
            None => Self::moa(),
        }
    }

    pub fn parse(data: ManagedBuffer<M>) -> Self {
        if data == Self::MOA_REPRESENTATION {
            Self::moa()
        } else {
            Self::dcdt(TokenIdentifier::from(data))
        }
    }

    #[inline]
    pub fn is_moa(&self) -> bool {
        self.data.is_none()
    }

    #[inline]
    pub fn is_dcdt(&self) -> bool {
        self.data.is_some()
    }

    #[inline]
    pub fn into_name(self) -> ManagedBuffer<M> {
        self.map_or_else(
            (),
            |()| ManagedBuffer::from(&Self::MOA_REPRESENTATION[..]),
            |(), token_identifier| token_identifier.into_managed_buffer(),
        )
    }

    /// Checks the DCDT token identifier for validity. MOA is considered valid, no checks needed.
    ///
    /// Will fail if it encodes an invalid DCDT token identifier.
    pub fn is_valid(&self) -> bool {
        self.map_ref_or_else(
            (),
            |()| true,
            |(), token_identifier| token_identifier.is_valid_dcdt_identifier(),
        )
    }

    pub fn map_or_else<Context, D, F, R>(self, context: Context, for_moa: D, for_dcdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, TokenIdentifier<M>) -> R,
    {
        self.data.map_or_else(context, for_moa, for_dcdt)
    }

    pub fn map_ref_or_else<Context, D, F, R>(&self, context: Context, for_moa: D, for_dcdt: F) -> R
    where
        D: FnOnce(Context) -> R,
        F: FnOnce(Context, &TokenIdentifier<M>) -> R,
    {
        self.data.map_ref_or_else(context, for_moa, for_dcdt)
    }

    pub fn unwrap_dcdt(self) -> TokenIdentifier<M> {
        self.data.unwrap_or_sc_panic("DCDT expected")
    }

    /// Representation of the object as an `Option`.
    ///
    /// Because it does not consume `self` only a reference to the DCDT token identifier can be returned.
    pub fn as_dcdt_option(&self) -> Option<ManagedRef<'_, M, TokenIdentifier<M>>> {
        self.data.as_option()
    }

    /// Converts `self` into an `Option`. Consumes `self` in the process.
    pub fn into_dcdt_option(self) -> Option<TokenIdentifier<M>> {
        self.data.into_option()
    }
}

impl<M: ManagedTypeApi> PartialEq for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.data == other.data
    }
}

impl<M: ManagedTypeApi> Eq for MoaOrDcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> PartialEq<TokenIdentifier<M>> for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn eq(&self, other: &TokenIdentifier<M>) -> bool {
        self.map_ref_or_else(
            (),
            |()| false,
            |(), self_dcdt_token_identifier| self_dcdt_token_identifier == other,
        )
    }
}

impl<M: ManagedTypeApi> NestedEncode for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn dep_encode_or_handle_err<O, H>(&self, dest: &mut O, h: H) -> Result<(), H::HandledErr>
    where
        O: NestedEncodeOutput,
        H: EncodeErrorHandler,
    {
        if let Some(token_identifier) = self.data.as_option() {
            token_identifier.dep_encode_or_handle_err(dest, h)
        } else {
            (&Self::MOA_REPRESENTATION[..]).dep_encode_or_handle_err(dest, h)
        }
    }
}

impl<M: ManagedTypeApi> TopEncode for MoaOrDcdtTokenIdentifier<M> {
    #[inline]
    fn top_encode_or_handle_err<O, H>(&self, output: O, h: H) -> Result<(), H::HandledErr>
    where
        O: TopEncodeOutput,
        H: EncodeErrorHandler,
    {
        if let Some(token_identifier) = self.data.as_option() {
            token_identifier.top_encode_or_handle_err(output, h)
        } else {
            (&Self::MOA_REPRESENTATION[..]).top_encode_or_handle_err(output, h)
        }
    }
}

impl<M: ManagedTypeApi> NestedDecode for MoaOrDcdtTokenIdentifier<M> {
    fn dep_decode_or_handle_err<I, H>(input: &mut I, h: H) -> Result<Self, H::HandledErr>
    where
        I: NestedDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::parse(ManagedBuffer::dep_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M: ManagedTypeApi> TopDecode for MoaOrDcdtTokenIdentifier<M> {
    fn top_decode_or_handle_err<I, H>(input: I, h: H) -> Result<Self, H::HandledErr>
    where
        I: TopDecodeInput,
        H: DecodeErrorHandler,
    {
        Ok(Self::parse(ManagedBuffer::top_decode_or_handle_err(
            input, h,
        )?))
    }
}

impl<M> TypeAbiFrom<TokenIdentifier<M>> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&TokenIdentifier<M>> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&[u8]> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}
impl<M> TypeAbiFrom<&str> for MoaOrDcdtTokenIdentifier<M> where M: ManagedTypeApi {}

impl<'a, M> TypeAbiFrom<TestTokenIdentifier<'a>> for MoaOrDcdtTokenIdentifier<M> where
    M: ManagedTypeApi
{
}
impl<'a, M> TypeAbiFrom<&TestTokenIdentifier<'a>> for MoaOrDcdtTokenIdentifier<M> where
    M: ManagedTypeApi
{
}

impl<M: ManagedTypeApi> TypeAbiFrom<Self> for MoaOrDcdtTokenIdentifier<M> {}
impl<M: ManagedTypeApi> TypeAbiFrom<&Self> for MoaOrDcdtTokenIdentifier<M> {}

impl<M: ManagedTypeApi> TypeAbi for MoaOrDcdtTokenIdentifier<M> {
    type Unmanaged = Self;

    fn type_name() -> TypeName {
        "MoaOrDcdtTokenIdentifier".into()
    }

    fn type_name_rust() -> TypeName {
        "MoaOrDcdtTokenIdentifier<$API>".into()
    }
}

impl<M: ManagedTypeApi> SCDisplay for MoaOrDcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer(&ManagedBuffer::from_handle(
                token_identifier.get_handle().cast_or_signal_error::<M, _>(),
            ));
        } else {
            f.append_bytes(Self::MOA_REPRESENTATION);
        }
    }
}

const MOA_REPRESENTATION_HEX: &[u8] = b"4d4f41";

impl<M: ManagedTypeApi> SCLowerHex for MoaOrDcdtTokenIdentifier<M> {
    fn fmt<F: FormatByteReceiver>(&self, f: &mut F) {
        if let Some(token_identifier) = self.data.as_option() {
            f.append_managed_buffer_lower_hex(&ManagedBuffer::from_handle(
                token_identifier.get_handle().cast_or_signal_error::<M, _>(),
            ));
        } else {
            f.append_bytes(MOA_REPRESENTATION_HEX);
        }
    }
}

impl<M> core::fmt::Debug for MoaOrDcdtTokenIdentifier<M>
where
    M: ManagedTypeApi,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if let Some(token_identifier) = self.data.as_option() {
            let token_id_str = token_identifier.to_string();
            f.debug_tuple("MoaOrDcdtTokenIdentifier::Dcdt")
                .field(&token_id_str)
                .finish()
        } else {
            f.write_str("MoaOrDcdtTokenIdentifier::Moa")
        }
    }
}
