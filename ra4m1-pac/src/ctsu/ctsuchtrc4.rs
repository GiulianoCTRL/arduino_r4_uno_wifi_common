#[doc = "Register `CTSUCHTRC4` reader"]
pub type R = crate::R<CTSUCHTRC4_SPEC>;
#[doc = "Register `CTSUCHTRC4` writer"]
pub type W = crate::W<CTSUCHTRC4_SPEC>;
#[doc = "Field `CTSUCHAC4` reader - CTSU Channel Transmit/Receive Control 4"]
pub type CTSUCHAC4_R = crate::FieldReader<CTSUCHAC4_A>;
#[doc = "CTSU Channel Transmit/Receive Control 4\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CTSUCHAC4_A {
    #[doc = "0: Reception"]
    _0 = 0,
    #[doc = "1: Transmission"]
    _1 = 1,
}
impl From<CTSUCHAC4_A> for u8 {
    #[inline(always)]
    fn from(variant: CTSUCHAC4_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CTSUCHAC4_A {
    type Ux = u8;
}
impl CTSUCHAC4_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<CTSUCHAC4_A> {
        match self.bits {
            0 => Some(CTSUCHAC4_A::_0),
            1 => Some(CTSUCHAC4_A::_1),
            _ => None,
        }
    }
    #[doc = "Reception"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCHAC4_A::_0
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCHAC4_A::_1
    }
}
#[doc = "Field `CTSUCHAC4` writer - CTSU Channel Transmit/Receive Control 4"]
pub type CTSUCHAC4_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, CTSUCHAC4_A>;
impl<'a, REG, const O: u8> CTSUCHAC4_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Reception"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC4_A::_0)
    }
    #[doc = "Transmission"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCHAC4_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - CTSU Channel Transmit/Receive Control 4"]
    #[inline(always)]
    pub fn ctsuchac4(&self) -> CTSUCHAC4_R {
        CTSUCHAC4_R::new(self.bits & 0x0f)
    }
}
impl W {
    #[doc = "Bits 0:3 - CTSU Channel Transmit/Receive Control 4"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuchac4(&mut self) -> CTSUCHAC4_W<CTSUCHTRC4_SPEC, 0> {
        CTSUCHAC4_W::new(self)
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
#[doc = "CTSU Channel Transmit/Receive Control Register 4\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsuchtrc4::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsuchtrc4::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUCHTRC4_SPEC;
impl crate::RegisterSpec for CTSUCHTRC4_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsuchtrc4::R`](R) reader structure"]
impl crate::Readable for CTSUCHTRC4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsuchtrc4::W`](W) writer structure"]
impl crate::Writable for CTSUCHTRC4_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCHTRC4 to value 0"]
impl crate::Resettable for CTSUCHTRC4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
