#[doc = "Register `RMINAR` reader"]
pub type R = crate::R<RMINAR_SPEC>;
#[doc = "Register `RMINAR` writer"]
pub type W = crate::W<RMINAR_SPEC>;
#[doc = "Field `MIN1` reader - 1-Minute Count Value for the ones place of minutes"]
pub type MIN1_R = crate::FieldReader;
#[doc = "Field `MIN1` writer - 1-Minute Count Value for the ones place of minutes"]
pub type MIN1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `MIN10` reader - 10-Minute Count Value for the tens place of minutes"]
pub type MIN10_R = crate::FieldReader;
#[doc = "Field `MIN10` writer - 10-Minute Count Value for the tens place of minutes"]
pub type MIN10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ENB` reader - Compare enable"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: The register value is not compared with the RMINCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RMINCNT counter value."]
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
    #[doc = "The register value is not compared with the RMINCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "The register value is compared with the RMINCNT counter value."]
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
    #[doc = "The register value is not compared with the RMINCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    #[doc = "The register value is compared with the RMINCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Minute Count Value for the ones place of minutes"]
    #[inline(always)]
    pub fn min1(&self) -> MIN1_R {
        MIN1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Minute Count Value for the tens place of minutes"]
    #[inline(always)]
    pub fn min10(&self) -> MIN10_R {
        MIN10_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Minute Count Value for the ones place of minutes"]
    #[inline(always)]
    #[must_use]
    pub fn min1(&mut self) -> MIN1_W<RMINAR_SPEC, 0> {
        MIN1_W::new(self)
    }
    #[doc = "Bits 4:6 - 10-Minute Count Value for the tens place of minutes"]
    #[inline(always)]
    #[must_use]
    pub fn min10(&mut self) -> MIN10_W<RMINAR_SPEC, 4> {
        MIN10_W::new(self)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<RMINAR_SPEC, 7> {
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
#[doc = "Minute Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rminar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rminar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMINAR_SPEC;
impl crate::RegisterSpec for RMINAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rminar::R`](R) reader structure"]
impl crate::Readable for RMINAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rminar::W`](W) writer structure"]
impl crate::Writable for RMINAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RMINAR to value 0"]
impl crate::Resettable for RMINAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
