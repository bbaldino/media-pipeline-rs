# media-pipeline-rs

RTP media pipelines in rust

An (in-progress) implementation of RTP media pipelines built on top of
[data-pipeline-rs](https://github.com/bbaldino/data-pipeline/).

## Pipeline build example

```rust
PipelineBuilder::new()
    .demux(
        "RTP/RTCP demuxer",
        StaticDemuxer::new(vec![
            ConditionalPath {
                predicate: Box::new(looks_like_rtp),
                next: PipelineBuilder::new()
                    .attach_handler(
                        "rtp decrypt",
                        SrtpDecrypt {
                            config: srtp_config.clone(),
                            contexts: HashMap::new(),
                        },
                    )
                    .attach_handler(
                        "RTP parser",
                        RtpParser::new(stream_information.subscribe_to_pt_changes()),
                    )
                    .attach_handler(
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
                                .attach_handler("audio silence checker", AudioSilenceChecker)
                                .attach_handler("audio discarder", DiscardableDiscarder)
                                .build(),
                            PipelineBuilder::new()
                                .attach_handler("video discarder", DiscardableDiscarder)
                                .build(),
                        ),
                    )
                    .build(),
            },
            ConditionalPath {
                predicate: Box::new(looks_like_rtcp),
                next: PipelineBuilder::new()
                    .attach_handler(
                        "rtcp decrypt",
                        SrtcpDecrypt {
                            config: srtp_config.clone(),
                            contexts: HashMap::new(),
                        },
                    )
                    .attach_handler("RTCP parser", CompoundRtcpParser)
                    .attach_handler("RTCP termination", RtcpTermination)
                    .build(),
            },
        ]),
    )
    .build()
```
