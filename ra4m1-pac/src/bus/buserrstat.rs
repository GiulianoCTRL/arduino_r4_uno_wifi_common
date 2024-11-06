#[doc = "Register `BUS%sERRSTAT` reader"]
pub type R = crate::R<BUSERRSTAT_SPEC>;
#[doc = "Field `ACCSTAT` reader - Error Access Status The status at the time of the error"]
pub type ACCSTAT_R = crate::BitReader<ACCSTAT_A>;
#[doc = "Error Access Status The status at the time of the error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ACCSTAT_A {
    #[doc = "0: Read access"]
    _0 = 0,
    #[doc = "1: Write Access"]
    _1 = 1,
}
impl From<ACCSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: ACCSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl ACCSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ACCSTAT_A {
        match self.bits {
            false => ACCSTAT_A::_0,
            true => ACCSTAT_A::_1,
        }
    }
    #[doc = "Read access"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ACCSTAT_A::_0
    }
    #[doc = "Write Access"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ACCSTAT_A::_1
    }
}
#[doc = "Field `ERRSTAT` reader - Bus Error Status When bus error assert, error flag occurs."]
pub type ERRSTAT_R = crate::BitReader<ERRSTAT_A>;
#[doc = "Bus Error Status When bus error assert, error flag occurs.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRSTAT_A {
    #[doc = "0: No bus error occurred"]
    _0 = 0,
    #[doc = "1: Bus error occurred."]
    _1 = 1,
}
impl From<ERRSTAT_A> for bool {
    #[inline(always)]
    fn from(variant: ERRSTAT_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRSTAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ERRSTAT_A {
        match self.bits {
            false => ERRSTAT_A::_0,
            true => ERRSTAT_A::_1,
        }
    }
    #[doc = "No bus error occurred"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ERRSTAT_A::_0
    }
    #[doc = "Bus error occurred."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ERRSTAT_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Error Access Status The status at the time of the error"]
    #[inline(always)]
    pub fn accstat(&self) -> ACCSTAT_R {
        ACCSTAT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 7 - Bus Error Status When bus error assert, error flag occurs."]
    #[inline(always)]
    pub fn errstat(&self) -> ERRSTAT_R {
        ERRSTAT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[doc = "Bus Error Status Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`buserrstat::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSERRSTAT_SPEC;
impl crate::RegisterSpec for BUSERRSTAT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`buserrstat::R`](R) reader structure"]
impl crate::Readable for BUSERRSTAT_SPEC {}
#[doc = "`reset()` method sets BUS%sERRSTAT to value 0"]
impl crate::Resettable for BUSERRSTAT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
