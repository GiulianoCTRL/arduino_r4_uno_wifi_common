#[doc = "Register `FRMNUM` reader"]
pub type R = crate::R<FRMNUM_SPEC>;
#[doc = "Register `FRMNUM` writer"]
pub type W = crate::W<FRMNUM_SPEC>;
#[doc = "Field `FRNM` reader - Frame Number Latest frame number"]
pub type FRNM_R = crate::FieldReader<u16>;
#[doc = "Field `CRCE` reader - Receive Data Error"]
pub type CRCE_R = crate::BitReader<CRCE_A>;
#[doc = "Receive Data Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCE_A {
    #[doc = "0: No error"]
    _0 = 0,
    #[doc = "1: An error occurred"]
    _1 = 1,
}
impl From<CRCE_A> for bool {
    #[inline(always)]
    fn from(variant: CRCE_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CRCE_A {
        match self.bits {
            false => CRCE_A::_0,
            true => CRCE_A::_1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CRCE_A::_0
    }
    #[doc = "An error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CRCE_A::_1
    }
}
#[doc = "Field `CRCE` writer - Receive Data Error"]
pub type CRCE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CRCE_A>;
impl<'a, REG, const O: u8> CRCE_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CRCE_A::_0)
    }
    #[doc = "An error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CRCE_A::_1)
    }
}
#[doc = "Field `OVRN` reader - Overrun/Underrun Detection Status"]
pub type OVRN_R = crate::BitReader<OVRN_A>;
#[doc = "Overrun/Underrun Detection Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRN_A {
    #[doc = "0: No error"]
    _0 = 0,
    #[doc = "1: An error occurred"]
    _1 = 1,
}
impl From<OVRN_A> for bool {
    #[inline(always)]
    fn from(variant: OVRN_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> OVRN_A {
        match self.bits {
            false => OVRN_A::_0,
            true => OVRN_A::_1,
        }
    }
    #[doc = "No error"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == OVRN_A::_0
    }
    #[doc = "An error occurred"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == OVRN_A::_1
    }
}
#[doc = "Field `OVRN` writer - Overrun/Underrun Detection Status"]
pub type OVRN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, OVRN_A>;
impl<'a, REG, const O: u8> OVRN_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "No error"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(OVRN_A::_0)
    }
    #[doc = "An error occurred"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(OVRN_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:10 - Frame Number Latest frame number"]
    #[inline(always)]
    pub fn frnm(&self) -> FRNM_R {
        FRNM_R::new(self.bits & 0x07ff)
    }
    #[doc = "Bit 14 - Receive Data Error"]
    #[inline(always)]
    pub fn crce(&self) -> CRCE_R {
        CRCE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Overrun/Underrun Detection Status"]
    #[inline(always)]
    pub fn ovrn(&self) -> OVRN_R {
        OVRN_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 14 - Receive Data Error"]
    #[inline(always)]
    #[must_use]
    pub fn crce(&mut self) -> CRCE_W<FRMNUM_SPEC, 14> {
        CRCE_W::new(self)
    }
    #[doc = "Bit 15 - Overrun/Underrun Detection Status"]
    #[inline(always)]
    #[must_use]
    pub fn ovrn(&mut self) -> OVRN_W<FRMNUM_SPEC, 15> {
        OVRN_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Frame Number Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`frmnum::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`frmnum::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FRMNUM_SPEC;
impl crate::RegisterSpec for FRMNUM_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`frmnum::R`](R) reader structure"]
impl crate::Readable for FRMNUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`frmnum::W`](W) writer structure"]
impl crate::Writable for FRMNUM_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FRMNUM to value 0"]
impl crate::Resettable for FRMNUM_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
