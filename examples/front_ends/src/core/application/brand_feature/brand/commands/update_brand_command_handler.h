// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#pragma once

#include "brand/brand_dto.h"
#include "brand/commands/update_brand_command.h"
#include "front_ends_example_application_brand_export.h"

#include "repository/interface_brand_repository.h"
#include "result.h"
#include <QPromise>

using namespace FrontEnds;
using namespace FrontEnds::Contracts::DTO::Brand;
using namespace FrontEnds::Contracts::Repository;
using namespace FrontEnds::Contracts::CQRS::Brand::Commands;

namespace FrontEnds::Application::Features::Brand::Commands
{
class FRONT_ENDS_EXAMPLE_APPLICATION_BRAND_EXPORT UpdateBrandCommandHandler : public QObject

{
    Q_OBJECT
public:
    UpdateBrandCommandHandler(InterfaceBrandRepository *repository);
    Result<BrandDTO> handle(QPromise<Result<void>> &progressPromise, const UpdateBrandCommand &request);
    Result<BrandDTO> restore();

Q_SIGNALS:
    void brandUpdated(FrontEnds::Contracts::DTO::Brand::BrandDTO brandDto);
    void brandDetailsUpdated(int id);

private:
    InterfaceBrandRepository *m_repository;
    Result<BrandDTO> handleImpl(QPromise<Result<void>> &progressPromise, const UpdateBrandCommand &request);
    Result<BrandDTO> restoreImpl();
    Result<BrandDTO> saveOldState();
    Result<BrandDTO> m_undoState;
    static bool s_mappingRegistered;
    void registerMappings();
};

} // namespace FrontEnds::Application::Features::Brand::Commands