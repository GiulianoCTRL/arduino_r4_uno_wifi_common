#[doc = "Register `RADJ` reader"]
pub type R = crate::R<RADJ_SPEC>;
#[doc = "Register `RADJ` writer"]
pub type W = crate::W<RADJ_SPEC>;
#[doc = "Field `ADJ` reader - Adjustment Value These bits specify the adjustment value from the prescaler."]
pub type ADJ_R = crate::FieldReader;
#[doc = "Field `ADJ` writer - Adjustment Value These bits specify the adjustment value from the prescaler."]
pub type ADJ_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 6, O>;
#[doc = "Field `PMADJ` reader - Plus-Minus"]
pub type PMADJ_R = crate::FieldReader<PMADJ_A>;
#[doc = "Plus-Minus\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMADJ_A {
    #[doc = "0: Adjustment is not performed."]
    _00 = 0,
    #[doc = "1: Adjustment is performed by the addition to the prescaler."]
    _01 = 1,
    #[doc = "2: Adjustment is performed by the subtraction from the prescaler."]
    _10 = 2,
    #[doc = "3: Setting prohibited"]
    _11 = 3,
}
impl From<PMADJ_A> for u8 {
    #[inline(always)]
    fn from(variant: PMADJ_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PMADJ_A {
    type Ux = u8;
}
impl PMADJ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PMADJ_A {
        match self.bits {
            0 => PMADJ_A::_00,
            1 => PMADJ_A::_01,
            2 => PMADJ_A::_10,
            3 => PMADJ_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Adjustment is not performed."]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == PMADJ_A::_00
    }
    #[doc = "Adjustment is performed by the addition to the prescaler."]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == PMADJ_A::_01
    }
    #[doc = "Adjustment is performed by the subtraction from the prescaler."]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == PMADJ_A::_10
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == PMADJ_A::_11
    }
}
#[doc = "Field `PMADJ` writer - Plus-Minus"]
pub type PMADJ_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, PMADJ_A>;
impl<'a, REG, const O: u8> PMADJ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Adjustment is not performed."]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(PMADJ_A::_00)
    }
    #[doc = "Adjustment is performed by the addition to the prescaler."]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(PMADJ_A::_01)
    }
    #[doc = "Adjustment is performed by the subtraction from the prescaler."]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(PMADJ_A::_10)
    }
    #[doc = "Setting prohibited"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(PMADJ_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:5 - Adjustment Value These bits specify the adjustment value from the prescaler."]
    #[inline(always)]
    pub fn adj(&self) -> ADJ_R {
        ADJ_R::new(self.bits & 0x3f)
    }
    #[doc = "Bits 6:7 - Plus-Minus"]
    #[inline(always)]
    pub fn pmadj(&self) -> PMADJ_R {
        PMADJ_R::new((self.bits >> 6) & 3)
    }
}
impl W {
    #[doc = "Bits 0:5 - Adjustment Value These bits specify the adjustment value from the prescaler."]
    #[inline(always)]
    #[must_use]
    pub fn adj(&mut self) -> ADJ_W<RADJ_SPEC, 0> {
        ADJ_W::new(self)
    }
    #[doc = "Bits 6:7 - Plus-Minus"]
    #[inline(always)]
    #[must_use]
    pub fn pmadj(&mut self) -> PMADJ_W<RADJ_SPEC, 6> {
        PMADJ_W::new(self)
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
#[doc = "Time Error Adjustment Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`radj::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`radj::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RADJ_SPEC;
impl crate::RegisterSpec for RADJ_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`radj::R`](R) reader structure"]
impl crate::Readable for RADJ_SPEC {}
#[doc = "`write(|w| ..)` method takes [`radj::W`](W) writer structure"]
impl crate::Writable for RADJ_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RADJ to value 0"]
impl crate::Resettable for RADJ_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
