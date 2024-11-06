#[doc = "Register `CTSUCR0` reader"]
pub type R = crate::R<CTSUCR0_SPEC>;
#[doc = "Register `CTSUCR0` writer"]
pub type W = crate::W<CTSUCR0_SPEC>;
#[doc = "Field `CTSUSTRT` reader - CTSU Measurement Operation Start"]
pub type CTSUSTRT_R = crate::BitReader<CTSUSTRT_A>;
#[doc = "CTSU Measurement Operation Start\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSTRT_A {
    #[doc = "0: Measurement operation stops."]
    _0 = 0,
    #[doc = "1: Measurement operation starts."]
    _1 = 1,
}
impl From<CTSUSTRT_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSTRT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUSTRT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSUSTRT_A {
        match self.bits {
            false => CTSUSTRT_A::_0,
            true => CTSUSTRT_A::_1,
        }
    }
    #[doc = "Measurement operation stops."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSTRT_A::_0
    }
    #[doc = "Measurement operation starts."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSTRT_A::_1
    }
}
#[doc = "Field `CTSUSTRT` writer - CTSU Measurement Operation Start"]
pub type CTSUSTRT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSUSTRT_A>;
impl<'a, REG, const O: u8> CTSUSTRT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Measurement operation stops."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSTRT_A::_0)
    }
    #[doc = "Measurement operation starts."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSTRT_A::_1)
    }
}
#[doc = "Field `CTSUCAP` reader - CTSU Measurement Operation Start Trigger Select"]
pub type CTSUCAP_R = crate::BitReader<CTSUCAP_A>;
#[doc = "CTSU Measurement Operation Start Trigger Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUCAP_A {
    #[doc = "0: Software trigger."]
    _0 = 0,
    #[doc = "1: External trigger."]
    _1 = 1,
}
impl From<CTSUCAP_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUCAP_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUCAP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSUCAP_A {
        match self.bits {
            false => CTSUCAP_A::_0,
            true => CTSUCAP_A::_1,
        }
    }
    #[doc = "Software trigger."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUCAP_A::_0
    }
    #[doc = "External trigger."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUCAP_A::_1
    }
}
#[doc = "Field `CTSUCAP` writer - CTSU Measurement Operation Start Trigger Select"]
pub type CTSUCAP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSUCAP_A>;
impl<'a, REG, const O: u8> CTSUCAP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Software trigger."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCAP_A::_0)
    }
    #[doc = "External trigger."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUCAP_A::_1)
    }
}
#[doc = "Field `CTSUSNZ` reader - CTSU Wait State Power-Saving Enable"]
pub type CTSUSNZ_R = crate::BitReader<CTSUSNZ_A>;
#[doc = "CTSU Wait State Power-Saving Enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUSNZ_A {
    #[doc = "0: Power-saving function during wait state is disabled."]
    _0 = 0,
    #[doc = "1: Power-saving function during wait state is enabled."]
    _1 = 1,
}
impl From<CTSUSNZ_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUSNZ_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUSNZ_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSUSNZ_A {
        match self.bits {
            false => CTSUSNZ_A::_0,
            true => CTSUSNZ_A::_1,
        }
    }
    #[doc = "Power-saving function during wait state is disabled."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUSNZ_A::_0
    }
    #[doc = "Power-saving function during wait state is enabled."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUSNZ_A::_1
    }
}
#[doc = "Field `CTSUSNZ` writer - CTSU Wait State Power-Saving Enable"]
pub type CTSUSNZ_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSUSNZ_A>;
impl<'a, REG, const O: u8> CTSUSNZ_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Power-saving function during wait state is disabled."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSNZ_A::_0)
    }
    #[doc = "Power-saving function during wait state is enabled."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUSNZ_A::_1)
    }
}
#[doc = "Field `CTSUINIT` reader - CTSU Control Block Initialization"]
pub type CTSUINIT_R = crate::BitReader<CTSUINIT_A>;
#[doc = "CTSU Control Block Initialization\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSUINIT_A {
    #[doc = "0: Writing a 0 has no effect, this bit is read as 0."]
    _0 = 0,
    #[doc = "1: initializes the CTSU control block and registers."]
    _1 = 1,
}
impl From<CTSUINIT_A> for bool {
    #[inline(always)]
    fn from(variant: CTSUINIT_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSUINIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CTSUINIT_A {
        match self.bits {
            false => CTSUINIT_A::_0,
            true => CTSUINIT_A::_1,
        }
    }
    #[doc = "Writing a 0 has no effect, this bit is read as 0."]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == CTSUINIT_A::_0
    }
    #[doc = "initializes the CTSU control block and registers."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == CTSUINIT_A::_1
    }
}
#[doc = "Field `CTSUINIT` writer - CTSU Control Block Initialization"]
pub type CTSUINIT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, CTSUINIT_A>;
impl<'a, REG, const O: u8> CTSUINIT_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Writing a 0 has no effect, this bit is read as 0."]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUINIT_A::_0)
    }
    #[doc = "initializes the CTSU control block and registers."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(CTSUINIT_A::_1)
    }
}
impl R {
    #[doc = "Bit 0 - CTSU Measurement Operation Start"]
    #[inline(always)]
    pub fn ctsustrt(&self) -> CTSUSTRT_R {
        CTSUSTRT_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    pub fn ctsucap(&self) -> CTSUCAP_R {
        CTSUCAP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    pub fn ctsusnz(&self) -> CTSUSNZ_R {
        CTSUSNZ_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - CTSU Control Block Initialization"]
    #[inline(always)]
    pub fn ctsuinit(&self) -> CTSUINIT_R {
        CTSUINIT_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CTSU Measurement Operation Start"]
    #[inline(always)]
    #[must_use]
    pub fn ctsustrt(&mut self) -> CTSUSTRT_W<CTSUCR0_SPEC, 0> {
        CTSUSTRT_W::new(self)
    }
    #[doc = "Bit 1 - CTSU Measurement Operation Start Trigger Select"]
    #[inline(always)]
    #[must_use]
    pub fn ctsucap(&mut self) -> CTSUCAP_W<CTSUCR0_SPEC, 1> {
        CTSUCAP_W::new(self)
    }
    #[doc = "Bit 2 - CTSU Wait State Power-Saving Enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsusnz(&mut self) -> CTSUSNZ_W<CTSUCR0_SPEC, 2> {
        CTSUSNZ_W::new(self)
    }
    #[doc = "Bit 4 - CTSU Control Block Initialization"]
    #[inline(always)]
    #[must_use]
    pub fn ctsuinit(&mut self) -> CTSUINIT_W<CTSUCR0_SPEC, 4> {
        CTSUINIT_W::new(self)
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
#[doc = "CTSU Control Register 0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`ctsucr0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ctsucr0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CTSUCR0_SPEC;
impl crate::RegisterSpec for CTSUCR0_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`ctsucr0::R`](R) reader structure"]
impl crate::Readable for CTSUCR0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ctsucr0::W`](W) writer structure"]
impl crate::Writable for CTSUCR0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CTSUCR0 to value 0"]
impl crate::Resettable for CTSUCR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
