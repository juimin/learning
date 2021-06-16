import React, { Component } from 'react'
import googleMapsLoader from 'react-google-maps-loader'
import GoogleMap from "react-google-map"
import $ from "jquery"

const GOOGLE_MAPS_API_KEY = 'AIzaSyCLia2sil-gbDo9sow_dlQYdizk-ojLsTA'; // Change your api key

class MyComponent extends Component {
    constructor() {
        super()
        this.state = {
            map: null,
            markers: new Map(),
        }
    }

    componentDidMount() {
        const {googleMaps} = this.props
    }

    render() {
        const {googleMaps} = this.props
        var coordArray = [];
        if (this.props.disasters) {
            var disasterObject = {};
            disasterObject = this.props.disasters;
            var disasterArray = $.map(disasterObject, function(value, index) {
                return [value];
            });
            //console.log(disasterArray);
            disasterArray.forEach( function (item)
            {
                //console.log(item.lat);
                var newLocation = {
                  title: item.name,
                  position: {lat: parseFloat(item.lat), lng: parseFloat(item.long)},
                  onLoaded: (googleMaps, map, marker) => {
                          const infoWindow = new googleMaps.InfoWindow({
                            content: `
                              <div>
                                <h3>` + item.name + `<h3>
                                <div>
                                  ` + item.description + `
                                </div>
                              </div>
                            `,
                          })
                          googleMaps.event.addListener(marker, "mouseover", () => {
                            infoWindow.open(map, marker)
                          })

                          googleMaps.event.addListener(marker, "mouseout", () => {
                            infoWindow.close(map, marker)
                          })

                          googleMaps.event.addListener(marker, "dblclick", () => {
                            marker.setMap(null);
                          })
                    },
                };
            coordArray.push(newLocation);
            });
            console.log(coordArray);
            }
        //seattleObject is a hardcoded test marker
        var seattleObject = {
                  title: "Seattle",
                  position: {lat: 47.61, lng: -122.34},
                  onLoaded: (googleMaps, map, marker) => {
                    const infoWindow = new googleMaps.InfoWindow({
                      content: `
                        <div>
                          <h3>Seattle<h3>
                          <div>
                            this disaster
                          </div>
                        </div>
                      `,
                    })

                    googleMaps.event.addListener(marker, "mouseover", () => {
                      infoWindow.open(map, marker)
                    })

                    googleMaps.event.addListener(marker, "mouseout", () => {
                      infoWindow.close(map, marker)
                    })
                  },
                }

            //var seattleObject2 = coordArray[0];


        return (
          <div className="map">
            <GoogleMap
              googleMaps={googleMaps}
              coordinates={
                coordArray
            }
              center={{lat: 39.833, lng: -98.583}}
              zoom={4}
              onLoaded={(googleMaps, map) => {
                map.setMapTypeId(googleMaps.MapTypeId.SATELLITE)
              }}
            />
          </div>
        )
    }
}

export default googleMapsLoader(MyComponent, {
  key: GOOGLE_MAPS_API_KEY,
})
