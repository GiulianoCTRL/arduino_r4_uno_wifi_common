#[doc = "Register `BKRACR` reader"]
pub type R = crate::R<BKRACR_SPEC>;
#[doc = "Register `BKRACR` writer"]
pub type W = crate::W<BKRACR_SPEC>;
#[doc = "Field `BKRACS` reader - Backup Register Access Control Register"]
pub type BKRACS_R = crate::FieldReader<BKRACS_A>;
#[doc = "Backup Register Access Control Register\n\nValue on reset: 6"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BKRACS_A {
    #[doc = "0: Access control disable. When System clock source is SOSC or LOCO."]
    _000 = 0,
    #[doc = "6: Access control enable. System clock source is other than SOSC or LOCO."]
    _110 = 6,
}
impl From<BKRACS_A> for u8 {
    #[inline(always)]
    fn from(variant: BKRACS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BKRACS_A {
    type Ux = u8;
}
impl BKRACS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<BKRACS_A> {
        match self.bits {
            0 => Some(BKRACS_A::_000),
            6 => Some(BKRACS_A::_110),
            _ => None,
        }
    }
    #[doc = "Access control disable. When System clock source is SOSC or LOCO."]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == BKRACS_A::_000
    }
    #[doc = "Access control enable. System clock source is other than SOSC or LOCO."]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == BKRACS_A::_110
    }
}
#[doc = "Field `BKRACS` writer - Backup Register Access Control Register"]
pub type BKRACS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, BKRACS_A>;
impl<'a, REG, const O: u8> BKRACS_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Access control disable. When System clock source is SOSC or LOCO."]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(BKRACS_A::_000)
    }
    #[doc = "Access control enable. System clock source is other than SOSC or LOCO."]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(BKRACS_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - Backup Register Access Control Register"]
    #[inline(always)]
    pub fn bkracs(&self) -> BKRACS_R {
        BKRACS_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - Backup Register Access Control Register"]
    #[inline(always)]
    #[must_use]
    pub fn bkracs(&mut self) -> BKRACS_W<BKRACR_SPEC, 0> {
        BKRACS_W::new(self)
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
#[doc = "Backup Register Access Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`bkracr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`bkracr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BKRACR_SPEC;
impl crate::RegisterSpec for BKRACR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`bkracr::R`](R) reader structure"]
impl crate::Readable for BKRACR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`bkracr::W`](W) writer structure"]
impl crate::Writable for BKRACR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BKRACR to value 0x06"]
impl crate::Resettable for BKRACR_SPEC {
    const RESET_VALUE: Self::Ux = 0x06;
}
