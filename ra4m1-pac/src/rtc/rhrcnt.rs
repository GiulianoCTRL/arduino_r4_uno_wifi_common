#[doc = "Register `RHRCNT` reader"]
pub type R = crate::R<RHRCNT_SPEC>;
#[doc = "Register `RHRCNT` writer"]
pub type W = crate::W<RHRCNT_SPEC>;
#[doc = "Field `HR1` reader - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
pub type HR1_R = crate::FieldReader;
#[doc = "Field `HR1` writer - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
pub type HR1_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O>;
#[doc = "Field `HR10` reader - 10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
pub type HR10_R = crate::FieldReader;
#[doc = "Field `HR10` writer - 10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
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
impl R {
    #[doc = "Bits 0:3 - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    pub fn hr1(&self) -> HR1_R {
        HR1_R::new(self.bits & 0x0f)
    }
    #[doc = "Bits 4:5 - 10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
    #[inline(always)]
    pub fn hr10(&self) -> HR10_R {
        HR10_R::new((self.bits >> 4) & 3)
    }
    #[doc = "Bit 6 - Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    pub fn pm(&self) -> PM_R {
        PM_R::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:3 - 1-Hour Count Counts from 0 to 9 once per hour. When a carry is generated, 1 is added to the tens place."]
    #[inline(always)]
    #[must_use]
    pub fn hr1(&mut self) -> HR1_W<RHRCNT_SPEC, 0> {
        HR1_W::new(self)
    }
    #[doc = "Bits 4:5 - 10-Hour Count Counts from 0 to 2 once per carry from the ones place."]
    #[inline(always)]
    #[must_use]
    pub fn hr10(&mut self) -> HR10_W<RHRCNT_SPEC, 4> {
        HR10_W::new(self)
    }
    #[doc = "Bit 6 - Time Counter Setting for a.m./p.m."]
    #[inline(always)]
    #[must_use]
    pub fn pm(&mut self) -> PM_W<RHRCNT_SPEC, 6> {
        PM_W::new(self)
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
#[doc = "Hour Counter\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rhrcnt::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`rhrcnt::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RHRCNT_SPEC;
impl crate::RegisterSpec for RHRCNT_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`rhrcnt::R`](R) reader structure"]
impl crate::Readable for RHRCNT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rhrcnt::W`](W) writer structure"]
impl crate::Writable for RHRCNT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RHRCNT to value 0"]
impl crate::Resettable for RHRCNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
