from core.assetverwaltung.serializers.base import BaseViewSerializer
from core.bauwerksdiagnostik.models.rueckprallhammer import Rueckprallhammer


class RueckprallhammerSerializer(BaseViewSerializer):
    class Meta:
        model = Rueckprallhammer
        fields = "__all__"
