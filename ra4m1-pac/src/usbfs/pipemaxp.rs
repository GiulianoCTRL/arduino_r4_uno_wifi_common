#[doc = "Register `PIPEMAXP` reader"]
pub type R = crate::R<PIPEMAXP_SPEC>;
#[doc = "Register `PIPEMAXP` writer"]
pub type W = crate::W<PIPEMAXP_SPEC>;
#[doc = "Field `MXPS` reader - Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\]
and \\[2:0\\]
are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\]
are not provided.)"]
pub type MXPS_R = crate::FieldReader<u16>;
#[doc = "Field `MXPS` writer - Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\]
and \\[2:0\\]
are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\]
are not provided.)"]
pub type MXPS_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 9, O, u16>;
#[doc = "Field `DEVSEL` reader - Device Select"]
pub type DEVSEL_R = crate::FieldReader<DEVSEL_A>;
#[doc = "Device Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DEVSEL_A {
    #[doc = "0: Address 0000"]
    _0000 = 0,
    #[doc = "1: Address 0001"]
    _0001 = 1,
    #[doc = "2: Address 0010"]
    _0010 = 2,
    #[doc = "3: Address 0011"]
    _0011 = 3,
    #[doc = "4: Address 0100"]
    _0100 = 4,
    #[doc = "5: Address 0101"]
    _0101 = 5,
}
impl From<DEVSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DEVSEL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DEVSEL_A {
    type Ux = u8;
}
impl DEVSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<DEVSEL_A> {
        match self.bits {
            0 => Some(DEVSEL_A::_0000),
            1 => Some(DEVSEL_A::_0001),
            2 => Some(DEVSEL_A::_0010),
            3 => Some(DEVSEL_A::_0011),
            4 => Some(DEVSEL_A::_0100),
            5 => Some(DEVSEL_A::_0101),
            _ => None,
        }
    }
    #[doc = "Address 0000"]
    #[inline(always)]
    pub fn is_0000(&self) -> bool {
        *self == DEVSEL_A::_0000
    }
    #[doc = "Address 0001"]
    #[inline(always)]
    pub fn is_0001(&self) -> bool {
        *self == DEVSEL_A::_0001
    }
    #[doc = "Address 0010"]
    #[inline(always)]
    pub fn is_0010(&self) -> bool {
        *self == DEVSEL_A::_0010
    }
    #[doc = "Address 0011"]
    #[inline(always)]
    pub fn is_0011(&self) -> bool {
        *self == DEVSEL_A::_0011
    }
    #[doc = "Address 0100"]
    #[inline(always)]
    pub fn is_0100(&self) -> bool {
        *self == DEVSEL_A::_0100
    }
    #[doc = "Address 0101"]
    #[inline(always)]
    pub fn is_0101(&self) -> bool {
        *self == DEVSEL_A::_0101
    }
}
#[doc = "Field `DEVSEL` writer - Device Select"]
pub type DEVSEL_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 4, O, DEVSEL_A>;
impl<'a, REG, const O: u8> DEVSEL_W<'a, REG, O>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Address 0000"]
    #[inline(always)]
    pub fn _0000(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0000)
    }
    #[doc = "Address 0001"]
    #[inline(always)]
    pub fn _0001(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0001)
    }
    #[doc = "Address 0010"]
    #[inline(always)]
    pub fn _0010(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0010)
    }
    #[doc = "Address 0011"]
    #[inline(always)]
    pub fn _0011(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0011)
    }
    #[doc = "Address 0100"]
    #[inline(always)]
    pub fn _0100(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0100)
    }
    #[doc = "Address 0101"]
    #[inline(always)]
    pub fn _0101(self) -> &'a mut crate::W<REG> {
        self.variant(DEVSEL_A::_0101)
    }
}
impl R {
    #[doc = "Bits 0:8 - Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\]
and \\[2:0\\]
are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\]
are not provided.)"]
    #[inline(always)]
    pub fn mxps(&self) -> MXPS_R {
        MXPS_R::new(self.bits & 0x01ff)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    pub fn devsel(&self) -> DEVSEL_R {
        DEVSEL_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:8 - Maximum Packet Size PIPE1 and PIPE2: 1 byte (001h) to 256 bytes (100h) PIPE3 to PIPE5: 8 bytes (008h), 16 bytes (010h), 32 bytes (020h), 64 bytes (040h) (Bits \\[8:7\\]
and \\[2:0\\]
are not provided.) PIPE6 to PIPE9: 1 byte (001h) to 64 bytes (040h) (Bits \\[8:7\\]
are not provided.)"]
    #[inline(always)]
    #[must_use]
    pub fn mxps(&mut self) -> MXPS_W<PIPEMAXP_SPEC, 0> {
        MXPS_W::new(self)
    }
    #[doc = "Bits 12:15 - Device Select"]
    #[inline(always)]
    #[must_use]
    pub fn devsel(&mut self) -> DEVSEL_W<PIPEMAXP_SPEC, 12> {
        DEVSEL_W::new(self)
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
#[doc = "Pipe Maximum Packet Size Register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pipemaxp::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pipemaxp::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PIPEMAXP_SPEC;
impl crate::RegisterSpec for PIPEMAXP_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [`pipemaxp::R`](R) reader structure"]
impl crate::Readable for PIPEMAXP_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pipemaxp::W`](W) writer structure"]
impl crate::Writable for PIPEMAXP_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PIPEMAXP to value 0"]
impl crate::Resettable for PIPEMAXP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
