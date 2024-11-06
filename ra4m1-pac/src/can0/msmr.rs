#[doc = "Register `MSMR` reader"]
pub type R = crate::R<MSMR_SPEC>;
#[doc = "Register `MSMR` writer"]
pub type W = crate::W<MSMR_SPEC>;
#[doc = "Field `MBSM` reader - Mailbox Search Mode Select"]
pub type MBSM_R = crate::FieldReader<MBSM_A>;
#[doc = "Mailbox Search Mode Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MBSM_A {
    #[doc = "0: Receive mailbox search mode"]
    _00 = 0,
    #[doc = "1: Transmit mailbox search mode"]
    _01 = 1,
    #[doc = "2: Message lost search mode"]
    _10 = 2,
    #[doc = "3: Channel search mode"]
    _11 = 3,
}
impl From<MBSM_A> for u8 {
    #[inline(always)]
    fn from(variant: MBSM_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MBSM_A {
    type Ux = u8;
}
impl MBSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> MBSM_A {
        match self.bits {
            0 => MBSM_A::_00,
            1 => MBSM_A::_01,
            2 => MBSM_A::_10,
            3 => MBSM_A::_11,
            _ => unreachable!(),
        }
    }
    #[doc = "Receive mailbox search mode"]
    #[inline(always)]
    pub fn is_00(&self) -> bool {
        *self == MBSM_A::_00
    }
    #[doc = "Transmit mailbox search mode"]
    #[inline(always)]
    pub fn is_01(&self) -> bool {
        *self == MBSM_A::_01
    }
    #[doc = "Message lost search mode"]
    #[inline(always)]
    pub fn is_10(&self) -> bool {
        *self == MBSM_A::_10
    }
    #[doc = "Channel search mode"]
    #[inline(always)]
    pub fn is_11(&self) -> bool {
        *self == MBSM_A::_11
    }
}
#[doc = "Field `MBSM` writer - Mailbox Search Mode Select"]
pub type MBSM_W<'a, REG, const O: u8> = crate::FieldWriterSafe<'a, REG, 2, O, MBSM_A>;
impl<'a, REG, const O: u8> MBSM_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Receive mailbox search mode"]
    #[inline(always)]
    pub fn _00(self) -> &'a mut crate::W<REG> {
        self.variant(MBSM_A::_00)
    }
    #[doc = "Transmit mailbox search mode"]
    #[inline(always)]
    pub fn _01(self) -> &'a mut crate::W<REG> {
        self.variant(MBSM_A::_01)
    }
    #[doc = "Message lost search mode"]
    #[inline(always)]
    pub fn _10(self) -> &'a mut crate::W<REG> {
        self.variant(MBSM_A::_10)
    }
    #[doc = "Channel search mode"]
    #[inline(always)]
    pub fn _11(self) -> &'a mut crate::W<REG> {
        self.variant(MBSM_A::_11)
    }
}
impl R {
    #[doc = "Bits 0:1 - Mailbox Search Mode Select"]
    #[inline(always)]
    pub fn mbsm(&self) -> MBSM_R {
        MBSM_R::new(self.bits & 3)
    }
}
impl W {
    #[doc = "Bits 0:1 - Mailbox Search Mode Select"]
    #[inline(always)]
    #[must_use]
    pub fn mbsm(&mut self) -> MBSM_W<MSMR_SPEC, 0> {
        MBSM_W::new(self)
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
#[doc = "Mailbox Search Mode Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`msmr::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`msmr::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MSMR_SPEC;
impl crate::RegisterSpec for MSMR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [`msmr::R`](R) reader structure"]
impl crate::Readable for MSMR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`msmr::W`](W) writer structure"]
impl crate::Writable for MSMR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSMR to value 0"]
impl crate::Resettable for MSMR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
