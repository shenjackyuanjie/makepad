// missing deps so our windows-rs stripper doesnt have to traverse types
use { 
    crate::{ 
        windows::{  
            core::HRESULT,
            core::PCWSTR, 
            core::PCSTR, 
            Storage::Streams::{ 
                IContentTypeProvider,
                InputStreamOptions, 
                IRandomAccessStreamWithContentType,
                IInputStream,
                IOutputStream,
                IRandomAccessStream,
                IBuffer,
                UnicodeEncoding,
                ByteOrder,
                DataReaderLoadOperation,
                IDataReader,
                IDataReaderFactory,
                IDataReaderStatics,
                DataWriterStoreOperation,
                IDataWriterFactory,
                IDataWriter,
            },
            Devices::Midi::{ 
                IMidiMessage,
                IMidiMessageReceivedEventArgs,
                MidiMessageReceivedEventArgs,
                MidiMessageType,
                IMidiInPortStatics,
                IMidiInPort,
                IMidiOutPortStatics,
                IMidiOutPort,
            },
            Devices::Enumeration::{    
                EnclosureLocation,
                DeviceInformationUpdate,
                DeviceInformationKind,
                DeviceInformationPairing,
                DeviceInformationCollection,
                DeviceClass,
                DevicePairingResult, 
                DevicePairingProtectionLevel,
                DeviceThumbnail,
                IDeviceInformationPairing2,
                IDevicePairingSettings,  
                DevicePairingResult, 
                DevicePairingKinds,
                DeviceInformationCustomPairing,
                DeviceUnpairingResult,
                DeviceUnpairingResultStatus,
                DeviceWatcherStatus,
                IDevicePairingRequestedEventArgs,
                IDeviceWatcher,
                IDeviceUnpairingResult,
                DevicePairingResultStatus,
                DevicePairingRequestedEventArgs,
                IDeviceInformationCustomPairing,
                IDevicePairingResult,
                IDeviceInformationPairing,
                IDeviceInformationPairingStatics,
                IDeviceInformationPairingStatics2,
                IDeviceInformationPairing,
                DeviceWatcher,
                Panel,
                IEnclosureLocation,
                IEnclosureLocation2,
                IDeviceInformation,  
                IDeviceInformation2,
                IDeviceInformationUpdate,  
                IDeviceInformationUpdate2,  
                IDeviceInformationStatics,  
                IDeviceInformationStatics2,
            },
            Win32::Media::MediaFoundation::{
                IMFActivate,
                IMFMediaBuffer,
                IMFAttributes,
                MF_ATTRIBUTE_TYPE, 
                MEDIA_EVENT_GENERATOR_GET_EVENT_FLAGS,
                IMFAsyncCallback,
                IMFAsyncResult,
                IMFPresentationDescriptor, 
                IMFMediaEventGenerator,
                MF_ATTRIBUTES_MATCH_TYPE,
                MF_SOURCE_READER_CONSTANTS,
                IMFStreamDescriptor,
                IMFMediaTypeHandler  
            },
            Win32::Graphics::Direct3D::{ 
                ID3DInclude, 
                D3D_SHADER_MACRO,
                D3D_SRV_DIMENSION,
                D3D_INCLUDE_TYPE,
                D3D_FEATURE_LEVEL,
                D3D_PRIMITIVE_TOPOLOGY,
                D3D_DRIVER_TYPE,
                
            },
            Win32::Graphics::Direct3D11::{
                ID3DInclude,
                ID3D11Asynchronous,
                ID3D11ClassInstance,
                ID3D11Texture3D,
                ID3D11Texture1D,
                ID3D11Resource,
                ID3D11UnorderedAccessView,
                ID3D11ClassLinkage,
                ID3D11GeometryShader,
                ID3D11HullShader,
                ID3D11DomainShader,
                ID3D11ComputeShader,
                ID3D11SamplerState,
                ID3D11DeviceChild,
                ID3D11View,
                ID3D11Query,
                ID3D11Predicate,
                ID3D11Counter,
                ID3D11CommandList,
                D3D11_USAGE,
                D3D11_DEPTH_WRITE_MASK,
                D3D11_COMPARISON_FUNC,
                D3D11_STENCIL_OP,
                D3D11_DSV_DIMENSION,
                D3D11_BLEND,
                D3D11_BLEND_OP,
                D3D11_CLEAR_FLAG,
                D3D11_COLOR_WRITE_ENABLE,
                D3D11_FILL_MODE,
                D3D11_CULL_MODE,
                D3D11_INPUT_CLASSIFICATION,
                
                D3D11_DEVICE_CONTEXT_TYPE,
                D3D11_FEATURE,
                D3D11_COUNTER,
                D3D11_COUNTER_INFO,
                D3D11_COUNTER_TYPE,
                D3D11_COUNTER_DESC,
                D3D11_RTV_DIMENSION,
                D3D11_BUFFER_RTV_0,
                D3D11_BUFFER_RTV_1,
                D3D11_BUFFER_RTV,
                D3D11_TEX1D_RTV,
                D3D11_TEX1D_ARRAY_RTV,
                D3D11_TEX2D_RTV,
                D3D11_TEX2D_ARRAY_RTV,
                D3D11_TEX2DMS_RTV,
                D3D11_TEX2DMS_ARRAY_RTV,
                D3D11_TEX3D_RTV,
                D3D11_MAP,
                D3D11_MAPPED_SUBRESOURCE,
                D3D11_BOX,
                D3D11_CPU_ACCESS_FLAG,
                
                D3D11_RENDER_TARGET_VIEW_DESC_0,
                D3D11_QUERY,
                D3D11_CLASS_INSTANCE_DESC,
                D3D11_UAV_DIMENSION,
                D3D11_BUFFER_UAV,
                D3D11_TEX1D_UAV,
                D3D11_TEX1D_ARRAY_UAV,
                D3D11_TEX2D_UAV,
                D3D11_TEX2D_ARRAY_UAV,
                D3D11_TEX3D_UAV,
                D3D11_BUFFER_SRV,
                D3D11_TEX1D_SRV,
                D3D11_TEX1D_ARRAY_SRV,
                D3D11_TEX2D_SRV,
                D3D11_TEX2D_ARRAY_SRV,
                D3D11_TEX2DMS_SRV,
                D3D11_TEX2DMS_ARRAY_SRV,
                D3D11_TEX3D_SRV,
                D3D11_TEXCUBE_SRV,
                D3D11_TEXCUBE_ARRAY_SRV,
                D3D11_TEX1D_ARRAY_UAV,
                D3D11_BUFFEREX_SRV,
                D3D11_TEX1D_DSV,
                D3D11_TEX2D_DSV,
                D3D11_BUFFER_SRV_0,
                D3D11_BUFFER_SRV_1,
                D3D11_TEX1D_ARRAY_DSV,
                D3D11_TEX2D_ARRAY_DSV,
                D3D11_TEX2DMS_DSV,
                D3D11_TEX2DMS_ARRAY_DSV,
                D3D11_RENDER_TARGET_VIEW_DESC,
                D3D11_QUERY_DESC,
                D3D11_UNORDERED_ACCESS_VIEW_DESC,
                D3D11_UNORDERED_ACCESS_VIEW_DESC_0,
                D3D11_SHADER_RESOURCE_VIEW_DESC_0,
                D3D11_DEPTH_STENCIL_VIEW_DESC_0,
                D3D11_FILTER,
                D3D11_TEXTURE_ADDRESS_MODE,
                D3D11_TEXTURE1D_DESC,
                D3D11_TEXTURE3D_DESC,
                D3D11_SHADER_RESOURCE_VIEW_DESC,
                D3D11_SO_DECLARATION_ENTRY,
                D3D11_SAMPLER_DESC,
                D3D11_RESOURCE_DIMENSION,
            },
            Win32::Graphics::Dxgi::{
                Common::{
                    DXGI_MODE_DESC,
                    DXGI_RATIONAL,
                    DXGI_MODE_SCANLINE_ORDER,
                    DXGI_MODE_SCALING,
                    DXGI_MODE_ROTATION,
                    DXGI_OUTPUT_DESC,
                    DXGI_GAMMA_CONTROL_CAPABILITIES,
                    DXGI_GAMMA_CONTROL,
                    DXGI_RGB,
                    DXGI_ALPHA_MODE,
                },
                DXGI_USAGE,
                DXGI_RGBA,
                DXGI_PRESENT_PARAMETERS,
                DXGI_OUTPUT_DESC,
                DXGI_OUTPUT_DESC,
                DXGI_ADAPTER_DESC,
                DXGI_SURFACE_DESC,
                DXGI_MAPPED_RECT,
                DXGI_ADAPTER_DESC1,
                DXGI_SWAP_CHAIN_DESC,
                DXGI_FRAME_STATISTICS,
                DXGI_SWAP_CHAIN_FULLSCREEN_DESC,
                DXGI_SCALING,
                DXGI_SWAP_EFFECT, 
                IDXGIAdapter1,
                IDXGISurface,
                IDXGIOutput,
                IDXGIAdapter,
                IDXGIObject,
                IDXGIFactory,
                IDXGIDeviceSubObject,
                IDXGIFactory1,
                IDXGISwapChain,
            },
            Win32::Foundation::{
                WAIT_EVENT,
                HMODULE,
                VARIANT_BOOL, 
                VARIANT_FALSE, 
                VARIANT_TRUE, 
                COLORREF,
                WIN32_ERROR, 
                CHAR,
                FILETIME,
                LUID,
                POINT,
                HINSTANCE, 
                FARPROC,
                DECIMAL,
                DECIMAL_0,
                DECIMAL_1,
                DECIMAL_0_0,
                DECIMAL_1_0,
            },
            Win32::Graphics::Gdi::{
                HDC,  
                HRGN,
                HPEN,
                HPALETTE,
                HFONT,
                HBITMAP, 
                HGDIOBJ,
                HBRUSH,
                MONITOR_FROM_FLAGS,
                GET_DEVICE_CAPS_INDEX,
            },
            Win32::UI::WindowsAndMessaging::{
                WINDOWPLACEMENT_FLAGS,
                WNDCLASS_STYLES,
                PEEK_MESSAGE_REMOVE_TYPE,
                WINDOW_EX_STYLE,
                WINDOW_STYLE,
                WINDOW_LONG_PTR_INDEX,
                SHOW_WINDOW_CMD,
                SET_WINDOW_POS_FLAGS,
                SHOW_WINDOW_CMD,
                TIMERPROC,
                MSG,
                HMENU,
                WNDPROC,
                HCURSOR,
                HICON
            },
            Win32::UI::Input::KeyboardAndMouse {
                TRACKMOUSEEVENT_FLAGS,
            },
            Win32::UI::Shell::PropertiesSystem {
                IPropertyStore,
                PROPERTYKEY
            },
            Win32::Security::{
                SECURITY_ATTRIBUTES
            },
            Win32::System::Variant::{
                VARIANT,
                VARENUM,
                VARIANT_0,
                VARIANT_0_0,
                VARIANT_0_0_0,
                VARIANT_0_0_0_0,
            },
            Win32::System::Com::{
                COINIT,
                IMPLTYPEFLAGS,
                ITypeComp,
                IUNVOKEKIND,
                ITypeLib,
                TYPEATTR,
                FUNCDESC,
                VARDESC,
                IDispatch,
                IStream,
                ITypeInfo,
                ISequentialStream,
                StructuredStorage,
                CLSCTX,
                STGM,
                DESCKIND,
                BINDPTR,
                TYPEKIND,
                TYPEDESC,
                IDLDESC,
                ELEMDESC,
                FUNCKIND,
                INVOKEKIND,
                CALLCONV,
                FUNCFLAGS,
                VARDESC_0,
                TLIBATTR,
                DISPATCH_FLAGS,
                DISPPARAMS,
                EXCEPINFO,
                STREAM_SEEK,
                STGC,
                LOCKTYPE,
                STATSTG,
                STATFLAG,
                VARFLAGS,
                VARKIND,
                TYPEDESC_0,
                ELEMDESC_0,
                SYSKIND,
                LPEXCEPFINO_DEFERRED_FILLIN,
                IEnumSTATSTG,
                IDLFLAGS,
                SAFEARRAY,
                BLOB,
                CY,
                ADVANCED_FEATURE_FLAGS,
                SAFEARRAYBOUND,
                CY_0,
            },
            Win32::System::Ole::{ 
                CLIPBOARD_FORMAT,
                IRecordInfo,
                PARAMDESCEX,
                PARAMFLAGS,
                ARRAYDESC,
                PARAMDESC,
            },
            Win32::System::Com::StructuredStorage::{
                IEnumSTATSTG,
                CAC,
                CAPROPVARIANT,
                CAUB,
                CAUI,
                CASCODE,
                CACY,
                CACLIPDATA,
                CABSTR,
                CABSTRBLOB,
                CALPSTR,
                CALPWSTR,
                CAI,
                CAL,
                CAUL,
                CAH,
                CAUH,
                CAFLT,
                CADBL,
                CABOOL,
                CADATE,
                CAFILETIME,
                CACLSID,
                VERSIONEDSTREAM,
                IStorage,
                CLIPDATA,
                BSTRBLOB,
                VARENUM,
                PROPVARIANT,
                PROPVARIANT_0,
                PROPVARIANT_0_0,
                PROPVARIANT_0_0_0,
                STGMOVE,
            },
            Win32::Media::Audio::{
                MIDI_WAVE_OPEN_TYPE,
                WAVEFORMATEX,
                AUDCLNT_SHAREMODE,
                Identity,
                EDataFlow,
                IMMDeviceCollection,
                ERole,
                IMMDevice,
                IMMNotificationClient,
            },
        }
    }
}


