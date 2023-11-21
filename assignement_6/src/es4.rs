#[allow(dead_code)]
#[derive(Clone, Debug)]
struct PublicStreetLight {
    id: u32,
    on: bool,
    burn_out: bool,
}
impl PublicStreetLight {
    fn new(id: u32) -> Self {
        Self {
            id,
            on: false,
            burn_out: true,
        }
    }
}
impl Default for PublicStreetLight {
    fn default() -> Self {
        Self {
            id: 0,
            on: false,
            burn_out: false,
        }
    }
}

#[derive(Debug)]
struct PublicIllumination {
    lights: Vec<PublicStreetLight>,
}
impl PublicIllumination {
    fn new(lights: &[PublicStreetLight]) -> Self {
        Self {
            lights: lights.to_vec(),
        }
    }
}
impl Default for PublicIllumination {
    fn default() -> Self {
        Self { lights: Vec::new() }
    }
}
impl Iterator for PublicIllumination {
    type Item = PublicStreetLight;
    fn next(&mut self) -> Option<Self::Item> {
        self.lights
            .iter()
            .position(|light| light.burn_out)
            .map(|light| self.lights.remove(light))
    }
}

pub fn main_es4() {
    let light1 = PublicStreetLight::default();
    let light2 = PublicStreetLight::new(2);
    let light3 = PublicStreetLight::new(3);
    let light4 = PublicStreetLight::default();
    let light5 = PublicStreetLight::default();

    let illumination = PublicIllumination::new(&vec![light1, light2, light3, light4, light5]);

    println!("{:?}", illumination);
    for light in illumination {
        println!("{:?}", light);
    }
}
