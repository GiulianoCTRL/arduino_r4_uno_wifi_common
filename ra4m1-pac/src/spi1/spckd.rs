#[doc = "Register `SPCKD` reader"]
pub type R = crate::R<SPCKD_SPEC>;
#[doc = "Register `SPCKD` writer"]
pub type W = crate::W<SPCKD_SPEC>;
#[doc = "Field `SCKDL` reader - RSPCK Delay Setting"]
pub type SCKDL_R = crate::FieldReader<SCKDL_A>;
#[doc = "RSPCK Delay Setting\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SCKDL_A {
    #[doc = "0: 1 RSPCK"]
    _000 = 0,
    #[doc = "1: 2 RSPCK"]
    _001 = 1,
    #[doc = "2: 3 RSPCK"]
    _010 = 2,
    #[doc = "3: 4 RSPCK"]
    _011 = 3,
    #[doc = "4: 5 RSPCK"]
    _100 = 4,
    #[doc = "5: 6 RSPCK"]
    _101 = 5,
    #[doc = "6: 7 RSPCK"]
    _110 = 6,
    #[doc = "7: 8 RSPCK"]
    _111 = 7,
}
impl From<SCKDL_A> for u8 {
    #[inline(always)]
    fn from(variant: SCKDL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SCKDL_A {
    type Ux = u8;
}
impl SCKDL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> SCKDL_A {
        match self.bits {
            0 => SCKDL_A::_000,
            1 => SCKDL_A::_001,
            2 => SCKDL_A::_010,
            3 => SCKDL_A::_011,
            4 => SCKDL_A::_100,
            5 => SCKDL_A::_101,
            6 => SCKDL_A::_110,
            7 => SCKDL_A::_111,
            _ => unreachable!(),
        }
    }
    #[doc = "1 RSPCK"]
    #[inline(always)]
    pub fn is_000(&self) -> bool {
        *self == SCKDL_A::_000
    }
    #[doc = "2 RSPCK"]
    #[inline(always)]
    pub fn is_001(&self) -> bool {
        *self == SCKDL_A::_001
    }
    #[doc = "3 RSPCK"]
    #[inline(always)]
    pub fn is_010(&self) -> bool {
        *self == SCKDL_A::_010
    }
    #[doc = "4 RSPCK"]
    #[inline(always)]
    pub fn is_011(&self) -> bool {
        *self == SCKDL_A::_011
    }
    #[doc = "5 RSPCK"]
    #[inline(always)]
    pub fn is_100(&self) -> bool {
        *self == SCKDL_A::_100
    }
    #[doc = "6 RSPCK"]
    #[inline(always)]
    pub fn is_101(&self) -> bool {
        *self == SCKDL_A::_101
    }
    #[doc = "7 RSPCK"]
    #[inline(always)]
    pub fn is_110(&self) -> bool {
        *self == SCKDL_A::_110
    }
    #[doc = "8 RSPCK"]
    #[inline(always)]
    pub fn is_111(&self) -> bool {
        *self == SCKDL_A::_111
    }
}
#[doc = "Field `SCKDL` writer - RSPCK Delay Setting"]
pub type SCKDL_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 3, O, SCKDL_A>;
impl<'a, REG, const O: u8> SCKDL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "1 RSPCK"]
    #[inline(always)]
    pub fn _000(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_000)
    }
    #[doc = "2 RSPCK"]
    #[inline(always)]
    pub fn _001(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_001)
    }
    #[doc = "3 RSPCK"]
    #[inline(always)]
    pub fn _010(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_010)
    }
    #[doc = "4 RSPCK"]
    #[inline(always)]
    pub fn _011(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_011)
    }
    #[doc = "5 RSPCK"]
    #[inline(always)]
    pub fn _100(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_100)
    }
    #[doc = "6 RSPCK"]
    #[inline(always)]
    pub fn _101(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_101)
    }
    #[doc = "7 RSPCK"]
    #[inline(always)]
    pub fn _110(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_110)
    }
    #[doc = "8 RSPCK"]
    #[inline(always)]
    pub fn _111(self) -> &'a mut crate::W<REG> {
        self.variant(SCKDL_A::_111)
    }
}
impl R {
    #[doc = "Bits 0:2 - RSPCK Delay Setting"]
    #[inline(always)]
    pub fn sckdl(&self) -> SCKDL_R {
        SCKDL_R::new(self.bits & 7)
    }
}
impl W {
    #[doc = "Bits 0:2 - RSPCK Delay Setting"]
    #[inline(always)]
    #[must_use]
    pub fn sckdl(&mut self) -> SCKDL_W<SPCKD_SPEC, 0> {
        SCKDL_W::new(self)
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
#[doc = "SPI Clock Delay Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`spckd::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`spckd::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SPCKD_SPEC;
impl crate::RegisterSpec for SPCKD_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`spckd::R`](R) reader structure"]
impl crate::Readable for SPCKD_SPEC {}
#[doc = "`write(|w| ..)` method takes [`spckd::W`](W) writer structure"]
impl crate::Writable for SPCKD_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPCKD to value 0"]
impl crate::Resettable for SPCKD_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
