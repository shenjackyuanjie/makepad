use {
    std::collections::BTreeMap,
    std::ops::{ControlFlow, Try, FromResidual},
    crate::{
        makepad_platform::*,
        makepad_platform::audio::*,
        makepad_platform::midi::*,
       //audio_engine::AudioEngine
    }
};


// Audio component API



pub enum AudioComponentAction {}
pub trait AudioComponent: LiveApply {
    fn type_id(&self) -> LiveType where Self: 'static {LiveType::of::<Self>()}
    fn handle_event(&mut self, _cx: &mut Cx, event: &mut Event, _dispatch_action: &mut dyn FnMut(&mut Cx, AudioComponentAction));
    fn get_graph_node(&mut self, cx: &mut Cx) -> Box<dyn AudioGraphNode + Send>;
    fn audio_query(&mut self, _query: &AudioQuery, _callback: &mut Option<AudioQueryCb>) -> AudioResult;
}

pub trait AudioGraphNode {
    fn handle_midi_1_data(&mut self, data: Midi1Data);
    fn render_to_audio_buffer(&mut self, time: AudioTime, outputs: &mut [&mut AudioBuffer], inputs: &[&AudioBuffer]);
}

generate_ref_cast_api!(AudioComponent);


// Audio component registry


#[derive(Default, LiveComponentRegistry)]
pub struct AudioComponentRegistry {
    pub map: BTreeMap<LiveType, (LiveComponentInfo, Box<dyn AudioComponentFactory>)>
}

pub trait AudioComponentFactory {
    fn new(&self, cx: &mut Cx) -> Box<dyn AudioComponent>;
}


// Live bindings for AudioComponentRef

pub struct AudioQueryCb<'a>{
    pub cb:&'a mut dyn FnMut(&mut Box<dyn AudioComponent >)
}

impl<'a> AudioQueryCb<'a>{
    pub fn call(&mut self, args:&mut Box<dyn AudioComponent >){
        let cb = &mut self.cb;
        cb(args)
    }
}

pub struct AudioComponentRef(Option<Box<dyn AudioComponent >>);

impl AudioComponentRef {
    pub fn _as_ref(&mut self) -> Option<&Box<dyn AudioComponent >> {
        self.0.as_ref()
    }
    pub fn as_mut(&mut self) -> Option<&mut Box<dyn AudioComponent >> {
        self.0.as_mut()
    }
    
    pub fn audio_query(&mut self, query: &AudioQuery, callback: &mut Option<AudioQueryCb>) -> AudioResult {
        if let Some(inner) = &mut self.0 {
            match query {
                AudioQuery::TypeId(id) => {
                    if inner.type_id() == *id {
                        if let Some(callback) = callback {
                            callback.call(inner)
                        }
                        else {
                            return AudioResult::Found(inner)
                        }
                    }
                },
            }
            inner.audio_query(query, callback)
        }
        else {
            AudioResult::NotFound
        }
    }
}

impl LiveHook for AudioComponentRef {}
impl LiveApply for AudioComponentRef {
    fn apply(&mut self, cx: &mut Cx, from: ApplyFrom, index: usize, nodes: &[LiveNode]) -> usize {
        if let LiveValue::Class {live_type, ..} = nodes[index].value {
            if let Some(component) = &mut self.0 {
                if component.type_id() != live_type {
                    self.0 = None; // type changed, drop old component
                }
                else {
                    return component.apply(cx, from, index, nodes)
                }
            }
            if let Some(component) = cx.live_registry.clone().borrow()
                .components.get::<AudioComponentRegistry>().new(cx, live_type) {
                self.0 = Some(component);
                return self.0.as_mut().unwrap().apply(cx, from, index, nodes);
            }
        }
        else if let Some(component) = &mut self.0 {
            return component.apply(cx, from, index, nodes);
        }
        nodes.skip_node(index)
    }
}

impl LiveNew for AudioComponentRef {
    fn new(_cx: &mut Cx) -> Self {
        Self (None)
    }
    
    fn live_type_info(_cx: &mut Cx) -> LiveTypeInfo {
        LiveTypeInfo {
            module_id: LiveModuleId::from_str(&module_path!()).unwrap(),
            live_type: LiveType::of::<dyn AudioComponent>(),
            fields: Vec::new(),
            live_ignore: true,
            type_name: LiveId(0)
        }
    }
}

pub enum AudioQuery {
    TypeId(std::any::TypeId),
}

pub enum AudioResult<'a> {
    NotFound,
    Found(&'a mut Box<dyn AudioComponent>)
}

impl<'a> FromResidual for AudioResult<'a> {
    fn from_residual(residual: &'a mut Box<dyn AudioComponent>) -> Self {
        Self::Found(residual)
    }
}

impl<'a> Try for AudioResult<'a> {
    type Output = ();
    type Residual = &'a mut Box<dyn AudioComponent>;
    
    fn from_output(_: Self::Output) -> Self {
        AudioResult::NotFound
    }
    
    fn branch(self) -> ControlFlow<Self::Residual,
    Self::Output> {
        match self {
            Self::NotFound => ControlFlow::Continue(()),
            Self::Found(c) => ControlFlow::Break(c)
        }
    }
}

#[macro_export]
macro_rules!audio_component {
    ( $ ty: ident) => {
        | cx: &mut Cx | {
            struct Factory();
            impl AudioComponentFactory for Factory {
                fn new(&self, cx: &mut Cx) -> Box<dyn AudioComponent> {
                    Box::new( $ ty::new(cx))
                }
            }
            register_component_factory!(cx, AudioComponentRegistry, $ ty, Factory);
        }
    }
}
