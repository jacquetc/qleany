// This file was generated automatically by Qleany's generator, edit at your own risk!
// If you do, be careful to not overwrite it when you run the generator again.
#include "get_all_brand_query_handler.h"
#include "repository/interface_brand_repository.h"
#include "tools/automapper.h"

using namespace FrontEnds;
using namespace FrontEnds::Application::Features::Brand::Queries;

GetAllBrandQueryHandler::GetAllBrandQueryHandler(InterfaceBrandRepository *repository)
    : m_repository(repository)
{
    if (!s_mappingRegistered) {
        registerMappings();
        s_mappingRegistered = true;
    }
}

Result<QList<BrandDTO>> GetAllBrandQueryHandler::handle(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllBrandQueryHandler::handle called";

    Result<QList<BrandDTO>> result;

    try {
        result = handleImpl(progressPromise);
    } catch (const std::exception &ex) {
        result = Result<QList<BrandDTO>>(QLN_ERROR_2(Q_FUNC_INFO, Error::Critical, "Unknown error", ex.what()));
        qDebug() << "Error handling GetAllBrandQuery:" << ex.what();
    }
    progressPromise.addResult(Result<void>(result.error()));
    return result;
}

Result<QList<BrandDTO>> GetAllBrandQueryHandler::handleImpl(QPromise<Result<void>> &progressPromise)
{
    qDebug() << "GetAllBrandQueryHandler::handleImpl called";

    // do
    auto brandResult = m_repository->getAll();

    QLN_RETURN_IF_ERROR(QList<BrandDTO>, brandResult)

    // map
    QList<BrandDTO> dtoList;

    for (const FrontEnds::Entities::Brand &brand : brandResult.value()) {
        auto dto = FrontEnds::Tools::AutoMapper::map<FrontEnds::Entities::Brand, BrandDTO>(brand);
        dtoList.append(dto);
    }

    qDebug() << "GetAllBrandQueryHandler::handleImpl done";

    return Result<QList<BrandDTO>>(dtoList);
}

bool GetAllBrandQueryHandler::s_mappingRegistered = false;

void GetAllBrandQueryHandler::registerMappings()
{
    FrontEnds::Tools::AutoMapper::registerMapping<FrontEnds::Entities::Brand, Contracts::DTO::Brand::BrandDTO>(true, true);
}