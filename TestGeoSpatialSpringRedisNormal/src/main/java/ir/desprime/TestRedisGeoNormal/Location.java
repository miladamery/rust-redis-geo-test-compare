package ir.desprime.TestRedisGeoNormal;

import lombok.AllArgsConstructor;
import lombok.Data;

@Data
@AllArgsConstructor
public class Location {

    private String name;
    private Double lat;
    private Double lng;

}
