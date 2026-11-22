from core.assetverwaltung.serializers.base import BaseViewSerializer
from core.bauwerksdiagnostik.models.rissbreitenmessung import Rissbreitenmessung


class RissbreitenmessungSerializer(BaseViewSerializer):
    class Meta:
        model = Rissbreitenmessung
        fields = "__all__"
