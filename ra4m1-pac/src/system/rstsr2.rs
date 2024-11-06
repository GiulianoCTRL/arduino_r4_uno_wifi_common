#[doc = "Register `RSTSR2` reader"]
pub type R = crate::R<RSTSR2_SPEC>;
#[doc = "Register `RSTSR2` writer"]
pub type W = crate::W<RSTSR2_SPEC>;
#[doc = "Field `CWSF` reader - Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag.\n\nThe field is **modified** in some way after a read operation."]
pub type CWSF_R = crate::BitReader<CWSF_A>;
#[doc = "Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CWSF_A {
    #[doc = "0: Cold start"]
    _0 = 0,
    #[doc = "1: Warm start"]
    _1 = 1,
}
impl From<CWSF_A> for bool {
    #[inline(always)]
    fn from(variant: CWSF_A) -> Self {
        variant as u8 != 0
    }
}
impl CWSF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CWSF_A {
        match self.bits {
            false => CWSF_A::_0,
            true => CWSF_A::_1,
        }
    }
    #[doc = "Cold start"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CWSF_A::_0
    }
    #[doc = "Warm start"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CWSF_A::_1
    }
}
#[doc = "Field `CWSF` writer - Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag."]
pub type CWSF_W<'a, REG, const O: u8> = crate::BitWriter1S<'a, REG, O, CWSF_A>;
impl<'a, REG, const O: u8> CWSF_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Cold start"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CWSF_A::_0)
    }
    #[doc = "Warm start"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CWSF_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag."]
    #[inline(always)]
    pub fn cwsf(&self) -> CWSF_R {
        CWSF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cold/Warm Start Determination Flag Note: Only 1 can be written to set the flag."]
    #[inline(always)]
    #[must_use]
    pub fn cwsf(&mut self) -> CWSF_W<RSTSR2_SPEC, 0> {
        CWSF_W::new(self)
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
#[doc = "Reset Status Register 2\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rstsr2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rstsr2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSTSR2_SPEC;
impl crate::RegisterSpec for RSTSR2_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rstsr2::R`](R) reader structure"]
impl crate::Readable for RSTSR2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rstsr2::W`](W) writer structure"]
impl crate::Writable for RSTSR2_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
}
#[doc = "`reset()` method sets RSTSR2 to value 0"]
impl crate::Resettable for RSTSR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
