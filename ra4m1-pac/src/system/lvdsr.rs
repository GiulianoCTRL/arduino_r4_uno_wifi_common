#[doc = "Register `LVD%sSR` reader"]
pub type R = crate::R<LVDSR_SPEC>;
#[doc = "Register `LVD%sSR` writer"]
pub type W = crate::W<LVDSR_SPEC>;
#[doc = "Field `DET` reader - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.\n\nThe field is **modified** in some way after a read operation."]
pub type DET_R = crate::BitReader<DET_A>;
#[doc = "Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DET_A {
    #[doc = "0: Not detected"]
    _0 = 0,
    #[doc = "1: Vdet1 passage detection"]
    _1 = 1,
}
impl From<DET_A> for bool {
    #[inline(always)]
    fn from(variant: DET_A) -> Self {
        variant as u8 != 0
    }
}
impl DET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> DET_A {
        match self.bits {
            false => DET_A::_0,
            true => DET_A::_1,
        }
    }
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == DET_A::_0
    }
    #[doc = "Vdet1 passage detection"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == DET_A::_1
    }
}
#[doc = "Field `DET` writer - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
pub type DET_W<'a, REG, const O: u8> = crate::BitWriter0C<'a, REG, O, DET_A>;
impl<'a, REG, const O: u8> DET_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Not detected"]
    #[inline(always)]
    pub fn _0(self) -> &'a mut crate::W<REG> {
        self.variant(DET_A::_0)
    }
    #[doc = "Vdet1 passage detection"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut crate::W<REG> {
        self.variant(DET_A::_1)
    }
}
#[doc = "Field `MON` reader - Voltage Monitor 1 Signal Monitor Flag"]
pub type MON_R = crate::BitReader<MON_A>;
#[doc = "Voltage Monitor 1 Signal Monitor Flag\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MON_A {
    #[doc = "0: VCC &lt; Vdet"]
    _0 = 0,
    #[doc = "1: VCC >= Vdet or MON bit is disabled"]
    _1 = 1,
}
impl From<MON_A> for bool {
    #[inline(always)]
    fn from(variant: MON_A) -> Self {
        variant as u8 != 0
    }
}
impl MON_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MON_A {
        match self.bits {
            false => MON_A::_0,
            true => MON_A::_1,
        }
    }
    #[doc = "VCC &lt; Vdet"]
    #[inline(always)]
    pub fn is_0(&self) -> bool {
        *self == MON_A::_0
    }
    #[doc = "VCC >= Vdet or MON bit is disabled"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == MON_A::_1
    }
}
impl R {
    #[doc = "Bit 0 - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[inline(always)]
    pub fn det(&self) -> DET_R {
        DET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Voltage Monitor 1 Signal Monitor Flag"]
    #[inline(always)]
    pub fn mon(&self) -> MON_R {
        MON_R::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Voltage Monitor Voltage Change Detection Flag NOTE: Only 0 can be written to this bit. After writing 0 to this bit, it takes 2 system clock cycles for the bit to be read as 0."]
    #[inline(always)]
    #[must_use]
    pub fn det(&mut self) -> DET_W<LVDSR_SPEC, 0> {
        DET_W::new(self)
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
#[doc = "Voltage Monitor %s Circuit Status Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lvdsr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lvdsr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LVDSR_SPEC;
impl crate::RegisterSpec for LVDSR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`lvdsr::R`](R) reader structure"]
impl crate::Readable for LVDSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lvdsr::W`](W) writer structure"]
impl crate::Writable for LVDSR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0x01;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LVD%sSR to value 0x02"]
impl crate::Resettable for LVDSR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
