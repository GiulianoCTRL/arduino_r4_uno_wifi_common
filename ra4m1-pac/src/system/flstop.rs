#[doc = "Register `FLSTOP` reader"]
pub type R = crate::R<FLSTOP_SPEC>;
#[doc = "Register `FLSTOP` writer"]
pub type W = crate::W<FLSTOP_SPEC>;
#[doc = "Field `FLSTOP` reader - Selecting ON/OFF of the Flash Memory Operation"]
pub type FLSTOP_R = crate::BitReader<FLSTOP_A>;
#[doc = "Selecting ON/OFF of the Flash Memory Operation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSTOP_A {
    #[doc = "0: Code flash and data flash memory operates"]
    _0 = 0,
    #[doc = "1: Code flash and data flash memory stops."]
    _1 = 1,
}
impl From<FLSTOP_A> for bool {
    #[inline(always)]
    fn from(variant: FLSTOP_A) -> Self {
        variant as u8 != 0
    }
}
impl FLSTOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLSTOP_A {
        match self.bits {
            false => FLSTOP_A::_0,
            true => FLSTOP_A::_1,
        }
    }
    #[doc = "Code flash and data flash memory operates"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLSTOP_A::_0
    }
    #[doc = "Code flash and data flash memory stops."]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLSTOP_A::_1
    }
}
#[doc = "Field `FLSTOP` writer - Selecting ON/OFF of the Flash Memory Operation"]
pub type FLSTOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O, FLSTOP_A>;
impl<'a, REG, const O: u8> FLSTOP_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Code flash and data flash memory operates"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(FLSTOP_A::_0)
    }
    #[doc = "Code flash and data flash memory stops."]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(FLSTOP_A::_1)
    }
}
#[doc = "Field `FLSTPF` reader - Flash Memory Operation Status Flag"]
pub type FLSTPF_R = crate::BitReader<FLSTPF_A>;
#[doc = "Flash Memory Operation Status Flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FLSTPF_A {
    #[doc = "0: Transition completed"]
    _0 = 0,
    #[doc = "1: During transition (from the flash-stop-status to flash-operating-status or vice versa)"]
    _1 = 1,
}
impl From<FLSTPF_A> for bool {
    #[inline(always)]
    fn from(variant: FLSTPF_A) -> Self {
        variant as u8 != 0
    }
}
impl FLSTPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> FLSTPF_A {
        match self.bits {
            false => FLSTPF_A::_0,
            true => FLSTPF_A::_1,
        }
    }
    #[doc = "Transition completed"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == FLSTPF_A::_0
    }
    #[doc = "During transition (from the flash-stop-status to flash-operating-status or vice versa)"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLSTPF_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    pub fn flstop(&self) -> FLSTOP_R {
        FLSTOP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 4 - Flash Memory Operation Status Flag"]
    #[inline(always)]
    pub fn flstpf(&self) -> FLSTPF_R {
        FLSTPF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Selecting ON/OFF of the Flash Memory Operation"]
    #[inline(always)]
    #[must_use]
    pub fn flstop(&mut self) -> FLSTOP_W<FLSTOP_SPEC, 0> {
        FLSTOP_W::new(self)
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
#[doc = "Flash Operation Control Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`flstop::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`flstop::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FLSTOP_SPEC;
impl crate::RegisterSpec for FLSTOP_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`flstop::R`](R) reader structure"]
impl crate::Readable for FLSTOP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`flstop::W`](W) writer structure"]
impl crate::Writable for FLSTOP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets FLSTOP to value 0"]
impl crate::Resettable for FLSTOP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
