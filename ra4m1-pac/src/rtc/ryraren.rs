#[doc = "Register `RYRAREN` reader"]
pub type R = crate::R<RYRAREN_SPEC>;
#[doc = "Register `RYRAREN` writer"]
pub type W = crate::W<RYRAREN_SPEC>;
#[doc = "Field `ENB` reader - Compare enable"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: The register value is not compared with the RYRCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RYRCNT counter value."]
    _1 = 1,
}
impl From<ENB_A> for bool {
    #[inline(always)]
    fn from(variant: ENB_A) -> Self {
        variant as u8 != 0
    }
}
impl ENB_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ENB_A {
        match self.bits {
            false => ENB_A::_0,
            true => ENB_A::_1,
        }
    }
    #[doc = "The register value is not compared with the RYRCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "The register value is compared with the RYRCNT counter value."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == ENB_A::_1
    }
}
#[doc = "Field `ENB` writer - Compare enable"]
pub type ENB_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, ENB_A>;
impl<'a, REG, const O: u8> ENB_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "The register value is not compared with the RYRCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    #[doc = "The register value is compared with the RYRCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<RYRAREN_SPEC, 7> {
        ENB_W::new(self)
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
#[doc = "Year Alarm Enable Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ryraren::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ryraren::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RYRAREN_SPEC;
impl crate::RegisterSpec for RYRAREN_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ryraren::R`](R) reader structure"]
impl crate::Readable for RYRAREN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ryraren::W`](W) writer structure"]
impl crate::Writable for RYRAREN_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RYRAREN to value 0"]
impl crate::Resettable for RYRAREN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
