#[doc = "Register `DAVREFCR` reader"]
pub type R = crate::R<DAVREFCR_SPEC>;
#[doc = "Register `DAVREFCR` writer"]
pub type W = crate::W<DAVREFCR_SPEC>;
#[doc = "Field `REF` reader - D/A Reference Voltage Select"]
pub type REF_R = crate::FieldReader<REF_A>;
#[doc = "D/A Reference Voltage Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REF_A {
    #[doc = "0: Not selected"]
    _000 = 0,
    #[doc = "1: AVCC0/AVSS0"]
    _001 = 1,
    #[doc = "3: Internal reference voltage/AVSS0"]
    _011 = 3,
    #[doc = "6: VREFH/VREFL"]
    _110 = 6,
}
impl From<REF_A> for u8 {
    #[inline(always)]
    fn from(variant: REF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REF_A {
    type Ux = u8;
}
impl REF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<REF_A> {
        match self.bits {
            0 => Some(REF_A::_000),
            1 => Some(REF_A::_001),
            3 => Some(REF_A::_011),
            6 => Some(REF_A::_110),
            _ => None,
        }
    }
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == REF_A::_000
    }
    #[doc = "AVCC0/AVSS0"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == REF_A::_001
    }
    #[doc = "Internal reference voltage/AVSS0"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == REF_A::_011
    }
    #[doc = "VREFH/VREFL"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == REF_A::_110
    }
}
#[doc = "Field `REF` writer - D/A Reference Voltage Select"]
pub type REF_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O, REF_A>;
impl<'a, REG, const O: u8> REF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Not selected"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_000)
    }
    #[doc = "AVCC0/AVSS0"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_001)
    }
    #[doc = "Internal reference voltage/AVSS0"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_011)
    }
    #[doc = "VREFH/VREFL"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(REF_A::_110)
    }
}
impl R {
    #[doc = "Bits 0:2 - D/A Reference Voltage Select"]
    #[inline(always)]
    pub fn ref_(&self) -> REF_R {
        REF_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - D/A Reference Voltage Select"]
    #[inline(always)]
    #[must_use]
    pub fn ref_(&mut self) -> REF_W<DAVREFCR_SPEC, 0> {
        REF_W::new(self)
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
#[doc = "D/A VREF Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`davrefcr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`davrefcr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DAVREFCR_SPEC;
impl crate::RegisterSpec for DAVREFCR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`davrefcr::R`](R) reader structure"]
impl crate::Readable for DAVREFCR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`davrefcr::W`](W) writer structure"]
impl crate::Writable for DAVREFCR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DAVREFCR to value 0"]
impl crate::Resettable for DAVREFCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
