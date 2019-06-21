use std::fmt;

use crate::{
    dnn::DictValue,
    Result,
    sys,
};

impl DictValue {
    #[inline]
    pub fn get_f64(&self, idx: i32) -> Result<f64> {
        let me = self.as_raw_DictValue();
        cpp!(unsafe [me as "const dnn::DictValue*", idx as "int"] -> sys::cv_return_value_double as "cv_return_value_double" {
            try {
                return { Error::Code::StsOk, NULL, me->get<double>(idx) };
            } CVRS_CATCH(cv_return_value_double)
        }).into_result()
    }

    #[inline]
    pub fn get_i64(&self, idx: i32) -> Result<i64> {
        let me = self.as_raw_DictValue();
        cpp!(unsafe [me as "const dnn::DictValue*", idx as "int"] -> sys::cv_return_value_int64 as "cv_return_value_int64" {
            try {
                return { Error::Code::StsOk, NULL, me->get<int64>(idx) };
            } CVRS_CATCH(cv_return_value_int64)
        }).into_result()
    }

    #[inline]
    pub fn get_string(&self, idx: i32) -> Result<String> {
        let me = self.as_raw_DictValue();
        cpp!(unsafe [me as "const dnn::DictValue*", idx as "int"] -> sys::cv_return_value_char_X as "cv_return_value_char_X" {
            try {
                return { Error::Code::StsOk, NULL, strdup(me->get<String>(idx).c_str()) };
            } CVRS_CATCH(cv_return_value_char_X)
        }).into_result().map(crate::templ::receive_string_mut)
    }
}

impl fmt::Debug for DictValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut d = f.debug_struct("DictValue");
        d.field("is_int", &self.is_int().map_err(|_| fmt::Error)?)
            .field("is_real", &self.is_real().map_err(|_| fmt::Error)?)
            .field("is_string", &self.is_string().map_err(|_| fmt::Error)?);
        if self.is_string().map_err(|_| fmt::Error)? {
            d.field("value", &self.get_string(-1).map_err(|_| fmt::Error)?);
        } else if self.is_int().map_err(|_| fmt::Error)? {
            d.field("value", &self.get_i64(-1).map_err(|_| fmt::Error)?);
        } else if self.is_real().map_err(|_| fmt::Error)? {
            d.field("value", &self.get_f64(-1).map_err(|_| fmt::Error)?);
        }
        d.finish()
    }
}
