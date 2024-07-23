# media-pipeline-rs

RTP media pipelines in rust

A framework and implementation for stringing nodes together which perform
various operations on RTP/RTCP packets.

## Pipeline build example

```rust
PipelineBuilder::new()
    .demux(
        "RTP/RTCP demuxer",
        StaticDemuxer::new(vec![
            ConditionalPath {
                predicate: Box::new(looks_like_rtp),
                next: PipelineBuilder::new()
                    .attach(
                        "rtp decrypt",
                        SrtpDecrypt {
                            config: srtp_config.clone(),
                            contexts: HashMap::new(),
                        },
                    )
                    .attach(
                        "RTP parser",
                        RtpParser::new(stream_information.subscribe_to_pt_changes()),
                    )
                    .attach(
                        "TCC generator",
                        TccGenerator::new(
                            stream_information
                                .subscribe_to_header_extension_id_change(TCC_URI.to_owned()),
                        ),
                    )
                    .demux(
                        "A/V demuxer",
                        AvDemuxer::new(
                            PipelineBuilder::new()
                                .attach("audio silence checker", AudioSilenceChecker)
                                .attach("audio discarder", DiscardableDiscarder)
                                .build(),
                            PipelineBuilder::new()
                                .attach("video discarder", DiscardableDiscarder)
                                .build(),
                        ),
                    )
                    .build(),
            },
            ConditionalPath {
                predicate: Box::new(looks_like_rtcp),
                next: PipelineBuilder::new()
                    .attach(
                        "rtcp decrypt",
                        SrtcpDecrypt {
                            config: srtp_config.clone(),
                            contexts: HashMap::new(),
                        },
                    )
                    .attach("RTCP parser", CompoundRtcpParser)
                    .attach("RTCP termination", RtcpTermination)
                    .build(),
            },
        ]),
    )
    .build()
```
