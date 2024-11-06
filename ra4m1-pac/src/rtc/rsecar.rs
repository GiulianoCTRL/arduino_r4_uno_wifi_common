#[doc = "Register `RSECAR` reader"]
pub type R = crate::R<RSECAR_SPEC>;
#[doc = "Register `RSECAR` writer"]
pub type W = crate::W<RSECAR_SPEC>;
#[doc = "Field `SEC1` reader - 1-Second Value for the ones place of seconds"]
pub type SEC1_R = crate::FieldReader;
#[doc = "Field `SEC1` writer - 1-Second Value for the ones place of seconds"]
pub type SEC1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `SEC10` reader - 10-Seconds Value for the tens place of seconds"]
pub type SEC10_R = crate::FieldReader;
#[doc = "Field `SEC10` writer - 10-Seconds Value for the tens place of seconds"]
pub type SEC10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 3, O>;
#[doc = "Field `ENB` reader - Compare enable"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: The register value is not compared with the RSECCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RSECCNT counter value."]
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
    #[doc = "The register value is not compared with the RSECCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "The register value is compared with the RSECCNT counter value."]
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
    #[doc = "The register value is not compared with the RSECCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    #[doc = "The register value is compared with the RSECCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Second Value for the ones place of seconds"]
    #[inline(always)]
    pub fn sec1(&self) -> SEC1_R {
        SEC1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:6 - 10-Seconds Value for the tens place of seconds"]
    #[inline(always)]
    pub fn sec10(&self) -> SEC10_R {
        SEC10_R::new((self.bits >> 4) & 7)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Second Value for the ones place of seconds"]
    #[inline(always)]
    #[must_use]
    pub fn sec1(&mut self) -> SEC1_W<RSECAR_SPEC, 0> {
        SEC1_W::new(self)
    }
    #[doc = "Bits 4:6 - 10-Seconds Value for the tens place of seconds"]
    #[inline(always)]
    #[must_use]
    pub fn sec10(&mut self) -> SEC10_W<RSECAR_SPEC, 4> {
        SEC10_W::new(self)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<RSECAR_SPEC, 7> {
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
#[doc = "Second Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rsecar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rsecar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RSECAR_SPEC;
impl crate::RegisterSpec for RSECAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rsecar::R`](R) reader structure"]
impl crate::Readable for RSECAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rsecar::W`](W) writer structure"]
impl crate::Writable for RSECAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSECAR to value 0"]
impl crate::Resettable for RSECAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
