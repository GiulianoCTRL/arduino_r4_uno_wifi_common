#[doc = "Register `PIPESEL` reader"]
pub type R = crate::R<PIPESEL_SPEC>;
#[doc = "Register `PIPESEL` writer"]
pub type W = crate::W<PIPESEL_SPEC>;
#[doc = "Field `PIPESEL` reader - Pipe Window Select"]
pub type PIPESEL_R = crate::FieldReader<PIPESEL_A>;
#[doc = "Pipe Window Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PIPESEL_A {
    #[doc = "0: No pipe selected"]
    _0000 = 0,
    #[doc = "1: PIPE1"]
    _0001 = 1,
    #[doc = "2: PIPE2"]
    _0010 = 2,
    #[doc = "3: PIPE3"]
    _0011 = 3,
    #[doc = "4: PIPE4"]
    _0100 = 4,
    #[doc = "5: PIPE5"]
    _0101 = 5,
    #[doc = "6: PIPE6"]
    _0110 = 6,
    #[doc = "7: PIPE7"]
    _0111 = 7,
    #[doc = "8: PIPE8"]
    _1000 = 8,
    #[doc = "9: PIPE9"]
    _1001 = 9,
}
impl From<PIPESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: PIPESEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PIPESEL_A {
    type Ux = u8;
}
impl PIPESEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<PIPESEL_A> {
        match self.bits {
            0 => Some(PIPESEL_A::_0000),
            1 => Some(PIPESEL_A::_0001),
            2 => Some(PIPESEL_A::_0010),
            3 => Some(PIPESEL_A::_0011),
            4 => Some(PIPESEL_A::_0100),
            5 => Some(PIPESEL_A::_0101),
            6 => Some(PIPESEL_A::_0110),
            7 => Some(PIPESEL_A::_0111),
            8 => Some(PIPESEL_A::_1000),
            9 => Some(PIPESEL_A::_1001),
            _ => None,
        }
    }
    #[doc = "No pipe selected"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == PIPESEL_A::_0000
    }
    #[doc = "PIPE1"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == PIPESEL_A::_0001
    }
    #[doc = "PIPE2"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == PIPESEL_A::_0010
    }
    #[doc = "PIPE3"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == PIPESEL_A::_0011
    }
    #[doc = "PIPE4"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == PIPESEL_A::_0100
    }
    #[doc = "PIPE5"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == PIPESEL_A::_0101
    }
    #[doc = "PIPE6"]
    #[inline(always)]
    pub fn is_0110(&self) -> bool {
        *self == PIPESEL_A::_0110
    }
    #[doc = "PIPE7"]
    #[inline(always)]
    pub fn is_0111(&self) -> bool {
        *self == PIPESEL_A::_0111
    }
    #[doc = "PIPE8"]
    #[inline(always)]
    pub fn is_1000(&self) -> bool {
        *self == PIPESEL_A::_1000
    }
    #[doc = "PIPE9"]
    #[inline(always)]
    pub fn is_1001(&self) -> bool {
        *self == PIPESEL_A::_1001
    }
}
#[doc = "Field `PIPESEL` writer - Pipe Window Select"]
pub type PIPESEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, PIPESEL_A>;
impl<'a, REG, const O: u8> PIPESEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No pipe selected"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0000)
    }
    #[doc = "PIPE1"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0001)
    }
    #[doc = "PIPE2"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0010)
    }
    #[doc = "PIPE3"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0011)
    }
    #[doc = "PIPE4"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0100)
    }
    #[doc = "PIPE5"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0101)
    }
    #[doc = "PIPE6"]
    #[inline(always)]
    pub fn _0110(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0110)
    }
    #[doc = "PIPE7"]
    #[inline(always)]
    pub fn _0111(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_0111)
    }
    #[doc = "PIPE8"]
    #[inline(always)]
    pub fn _1000(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_1000)
    }
    #[doc = "PIPE9"]
    #[inline(always)]
    pub fn _1001(self) -> &'a mut crate::W<REG> {
        self.variant(PIPESEL_A::_1001)
    }
}
impl R {
    #[doc = "Bits 0:3 - Pipe Window Select"]
    #[inline(always)]
    pub fn pipesel(&self) -> PIPESEL_R {
        PIPESEL_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Pipe Window Select"]
    #[inline(always)]
    #[must_use]
    pub fn pipesel(&mut self) -> PIPESEL_W<PIPESEL_SPEC, 0> {
        PIPESEL_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Pipe Window Select Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipesel::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipesel::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIPESEL_SPEC;
impl crate::RegisterSpec for PIPESEL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipesel::R`](R) reader structure"]
impl crate::Readable for PIPESEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pipesel::W`](W) writer structure"]
impl crate::Writable for PIPESEL_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPESEL to value 0"]
impl crate::Resettable for PIPESEL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
