#[doc = "Register `BUSSCNT%s` reader"]
pub type R = crate::R<BUSSCNT_SPEC>;
#[doc = "Register `BUSSCNT%s` writer"]
pub type W = crate::W<BUSSCNT_SPEC>;
#[doc = "Field `ARBMET` reader - Arbitration Method Specify the priority between groups"]
pub type ARBMET_R = crate::FieldReader<ARBMET_A>;
#[doc = "Arbitration Method Specify the priority between groups\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ARBMET_A {
    #[doc = "0: fixed priority"]
    _00 = 0,
    #[doc = "1: round-robin"]
    _01 = 1,
}
impl From<ARBMET_A> for u8 {
    #[inline(always)]
    fn from(variant: ARBMET_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ARBMET_A {
    type Ux = u8;
}
impl ARBMET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<ARBMET_A> {
        match self.bits {
            0 => Some(ARBMET_A::_00),
            1 => Some(ARBMET_A::_01),
            _ => None,
        }
    }
    #[doc = "fixed priority"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == ARBMET_A::_00
    }
    #[doc = "round-robin"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == ARBMET_A::_01
    }
}
#[doc = "Field `ARBMET` writer - Arbitration Method Specify the priority between groups"]
pub type ARBMET_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O, ARBMET_A>;
impl<'a, REG, const O: u8> ARBMET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "fixed priority"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMET_A::_00)
    }
    #[doc = "round-robin"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(ARBMET_A::_01)
    }
}
impl R {
    #[doc = "Bits 4:5 - Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    pub fn arbmet(&self) -> ARBMET_R {
        ARBMET_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 4:5 - Arbitration Method Specify the priority between groups"]
    #[inline(always)]
    #[must_use]
    pub fn arbmet(&mut self) -> ARBMET_W<BUSSCNT_SPEC, 4> {
        ARBMET_W::new(self)
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
#[doc = "Slave Bus Control Register %s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`busscnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`busscnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BUSSCNT_SPEC;
impl crate::RegisterSpec for BUSSCNT_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`busscnt::R`](R) reader structure"]
impl crate::Readable for BUSSCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`busscnt::W`](W) writer structure"]
impl crate::Writable for BUSSCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BUSSCNT%s to value 0"]
impl crate::Resettable for BUSSCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
