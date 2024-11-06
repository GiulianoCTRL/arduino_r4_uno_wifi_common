#[doc = "Register `CTSUCHTRC1` reader"]
pub type R = crate::R<CTSUCHTRC1_SPEC>;
#[doc = "Register `CTSUCHTRC1` writer"]
pub type W = crate::W<CTSUCHTRC1_SPEC>;
#[doc = "Field `CTSUCHTRC1` reader - CTSU Channel Transmit/Receive Control 1"]
pub type CTSUCHTRC1_R = crate::FieldReader<CTSUCHTRC1_A>;
#[doc = "CTSU Channel Transmit/Receive Control 1\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHTRC1_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CTSUCHTRC1_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHTRC1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHTRC1_A {
    type Ux = u8;
}
impl CTSUCHTRC1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSUCHTRC1_A> {
        match self.bits {
            0 => Some(CTSUCHTRC1_A::_0),
            1 => Some(CTSUCHTRC1_A::_1),
            _ => None,
        }
    }
    #[doc = "Reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHTRC1_A::_0
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHTRC1_A::_1
    }
}
#[doc = "Field `CTSUCHTRC1` writer - CTSU Channel Transmit/Receive Control 1"]
pub type CTSUCHTRC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, CTSUCHTRC1_A>;
impl<'a, REG, const O: u8> CTSUCHTRC1_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC1_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHTRC1_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 1"]
    #[inline(always)]
    pub fn ctsuchtrc1(&self) -> CTSUCHTRC1_R {
        CTSUCHTRC1_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - CTSU Channel Transmit/Receive Control 1"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchtrc1(&mut self) -> CTSUCHTRC1_W<CTSUCHTRC1_SPEC, 0> {
        CTSUCHTRC1_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "CTSU Channel Transmit/Receive Control Register 1\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchtrc1::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchtrc1::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUCHTRC1_SPEC;
impl crate::RegisterSpec for CTSUCHTRC1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchtrc1::R`](R) reader structure"]
impl crate::Readable for CTSUCHTRC1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuchtrc1::W`](W) writer structure"]
impl crate::Writable for CTSUCHTRC1_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHTRC1 to value 0"]
impl crate::Resettable for CTSUCHTRC1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
