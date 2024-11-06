#[doc = "Register `RHRAR` reader"]
pub type R = crate::R<RHRAR_SPEC>;
#[doc = "Register `RHRAR` writer"]
pub type W = crate::W<RHRAR_SPEC>;
#[doc = "Field `HR1` reader - 1-Hour Count Value for the ones place of hours"]
pub type HR1_R = crate::FieldReader;
#[doc = "Field `HR1` writer - 1-Hour Count Value for the ones place of hours"]
pub type HR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HR10` reader - 10-Hour Count Value for the tens place of hours"]
pub type HR10_R = crate::FieldReader;
#[doc = "Field `HR10` writer - 10-Hour Count Value for the tens place of hours"]
pub type HR10_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 2, O>;
#[doc = "Field `PM` reader - Time Counter Setting for a.m./p.m."]
pub type PM_R = crate::BitReader<PM_A>;
#[doc = "Time Counter Setting for a.m./p.m.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PM_A {
    #[doc = "0: a.m."]
    _0 = 0,
    #[doc = "1: p.m."]
    _1 = 1,
}
impl From<PM_A> for bool {
    #[inline(always)]
    fn from(variant: PM_A) -> Self {
        variant as u8 != 0
    }
}
impl PM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> PM_A {
        match self.bits {
            false => PM_A::_0,
            true => PM_A::_1,
        }
    }
    #[doc = "a.m."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == PM_A::_0
    }
    #[doc = "p.m."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == PM_A::_1
    }
}
#[doc = "Field `PM` writer - Time Counter Setting for a.m./p.m."]
pub type PM_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, PM_A>;
impl<'a, REG, const O: u8> PM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "a.m."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_0)
    }
    #[doc = "p.m."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(PM_A::_1)
    }
}
#[doc = "Field `ENB` reader - Compare enable"]
pub type ENB_R = crate::BitReader<ENB_A>;
#[doc = "Compare enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ENB_A {
    #[doc = "0: The register value is not compared with the RHRCNT counter value."]
    _0 = 0,
    #[doc = "1: The register value is compared with the RHRCNT counter value."]
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
    #[doc = "The register value is not compared with the RHRCNT counter value."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == ENB_A::_0
    }
    #[doc = "The register value is compared with the RHRCNT counter value."]
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
    #[doc = "The register value is not compared with the RHRCNT counter value."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_0)
    }
    #[doc = "The register value is compared with the RHRCNT counter value."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(ENB_A::_1)
    }
}
impl R {
    #[doc = "Bits 0:3 - 1-Hour Count Value for the ones place of hours"]
    #[inline(always)]
    pub fn hr1(&self) -> HR1_R {
        HR1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Hour Count Value for the tens place of hours"]
    #[inline(always)]
    pub fn hr10(&self) -> HR10_R {
        HR10_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    pub fn enb(&self) -> ENB_R {
        ENB_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Hour Count Value for the ones place of hours"]
    #[inline(always)]
    #[must_use]
    pub fn hr1(&mut self) -> HR1_W<RHRAR_SPEC, 0> {
        HR1_W::new(self)
    }
    #[doc = "Bits 4:5 - 10-Hour Count Value for the tens place of hours"]
    #[inline(always)]
    #[must_use]
    pub fn hr10(&mut self) -> HR10_W<RHRAR_SPEC, 4> {
        HR10_W::new(self)
    }
    #[doc = "Bit 6 - Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<RHRAR_SPEC, 6> {
        PM_W::new(self)
    }
    #[doc = "Bit 7 - Compare enable"]
    #[inline(always)]
    #[must_use]
    pub fn enb(&mut self) -> ENB_W<RHRAR_SPEC, 7> {
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
#[doc = "Hour Alarm Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhrar::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rhrar::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RHRAR_SPEC;
impl crate::RegisterSpec for RHRAR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rhrar::R`](R) reader structure"]
impl crate::Readable for RHRAR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rhrar::W`](W) writer structure"]
impl crate::Writable for RHRAR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RHRAR to value 0"]
impl crate::Resettable for RHRAR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
