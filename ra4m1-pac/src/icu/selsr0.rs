#[doc = "Register `SELSR0` reader"]
pub type R = crate::R<SELSR0_SPEC>;
#[doc = "Register `SELSR0` writer"]
pub type W = crate::W<SELSR0_SPEC>;
#[doc = "Field `SELS` reader - SYS Event Link Select"]
pub type SELS_R = crate::FieldReader<SELS_A>;
#[doc = "SYS Event Link Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELS_A {
    #[doc = "0: Disable event output to the associated low-power mode module"]
    _0X00 = 0,
}
impl From<SELS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SELS_A {
    type Ux = u8;
}
impl SELS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<SELS_A> {
        match self.bits {
            0 => Some(SELS_A::_0X00),
            _ => None,
        }
    }
    #[doc = "Disable event output to the associated low-power mode module"]
    #[inline(always)]
    pub fn is_0x00(&self) -> bool {
        *self == SELS_A::_0X00
    }
}
#[doc = "Field `SELS` writer - SYS Event Link Select"]
pub type SELS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 8, O, SELS_A>;
impl<'a, REG, const O: u8> SELS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Disable event output to the associated low-power mode module"]
    #[inline(always)]
    pub fn _0x00(self) -> &'a mut crate::W<REG> {
        self.variant(SELS_A::_0X00)
    }
}
impl R {
    #[doc = "Bits 0:7 - SYS Event Link Select"]
    #[inline(always)]
    pub fn sels(&self) -> SELS_R {
        SELS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - SYS Event Link Select"]
    #[inline(always)]
    #[must_use]
    pub fn sels(&mut self) -> SELS_W<SELSR0_SPEC, 0> {
        SELS_W::new(self)
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
#[doc = "SYS Event Link Setting Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`selsr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`selsr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SELSR0_SPEC;
impl crate::RegisterSpec for SELSR0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`selsr0::R`](R) reader structure"]
impl crate::Readable for SELSR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`selsr0::W`](W) writer structure"]
impl crate::Writable for SELSR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SELSR0 to value 0"]
impl crate::Resettable for SELSR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
